
#!/usr/bin/env bash
# ==============================================================================
# Multi-Platform Cross-Compilation & Verification Script
# ==============================================================================
#
# Description:
#   Installs Rust compilation targets, runs cargo check for every available
#   target, and builds release binaries where the host environment supports
#   native compilation or configured cross-compilation.
#
# Requirements:
#   General:
#     - Bash 4.0+
#     - Rust toolchain (rustc, cargo)
#     - rustup
#     - Internet connection for downloading Rust targets
#
#   Linux:
#     - Zig
#     - cargo-zigbuild
#     - Native C toolchain and system development libraries
#     - Zig is required for Linux -> Windows GNU cross-compilation.
#
#   macOS:
#     - Xcode Command Line Tools
#     - Apple SDK
#     - Native Apple toolchain
#
#   FreeBSD:
#     - Native FreeBSD toolchain
#     - Required system development libraries and headers
#
#   OpenBSD:
#     - Native OpenBSD toolchain
#     - Required system development libraries and headers
#
#   NetBSD:
#     - Native NetBSD toolchain
#     - Required system development libraries and headers
#
# Build support:
#   Linux:
#     - x86_64-unknown-linux-gnu (native)
#     - x86_64-unknown-linux-musl
#     - x86_64-pc-windows-gnu (cross-compilation via Zig)
#
#   macOS:
#     - x86_64-apple-darwin (native x86_64)
#     - aarch64-apple-darwin (native Apple Silicon)
#
#   FreeBSD:
#     - x86_64-unknown-freebsd (native x86_64)
#
#   OpenBSD:
#     - x86_64-unknown-openbsd (native x86_64)
#
#   NetBSD:
#     - x86_64-unknown-netbsd (native x86_64)
#
# Cross-compilation limitations:
#   Linux can run cargo check for macOS and BSD targets, but cannot reliably
#   link their release binaries without the corresponding native toolchains,
#   system libraries, SDKs, and target sysroots.
#
#   macOS binaries require Apple's SDK and native linking environment.
#   These components are not provided by a standard Linux installation.
#
#   BSD binaries require platform-specific headers, libraries, sysroots,
#   and compatible linkers.
#
#   cargo check validates Rust source code without performing final binary
#   linking. It can therefore succeed even when the corresponding release
#   build cannot be linked on the current host.
#
#   cargo check may still require target-specific dependencies, build scripts,
#   native libraries, and other compilation prerequisites.
#
#   Unsupported build targets are automatically skipped according to the
#   detected host operating system and architecture.
#
#   Native targets use cargo build.
#   Linux-to-Windows cross-compilation uses cargo zigbuild.
#
# Usage:
#   ./build-all.sh [mode] [options]
#
# Modes:
#   check     - Run cargo check for every available target
#   build     - Build supported release binaries
#   all       - Run check, then build (default)
#
# Options:
#   -d, --details   Print every error and warning after each summary
#
# Output:
#   logs/
#     check-<target>.log
#     build-<target>.log
# ==============================================================================

set -uo pipefail

TARGETS=(
    # Linux
    "x86_64-unknown-linux-gnu"
    "x86_64-unknown-linux-musl"

    # Windows
    "x86_64-pc-windows-gnu"

    # macOS
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"

    # BSD
    "x86_64-unknown-freebsd"
    "x86_64-unknown-openbsd"
    "x86_64-unknown-netbsd"
)

MODE="all"
SHOW_DETAILS=0
LOG_DIR="logs"

HOST_OS=""
HOST_ARCH=""

AVAILABLE=()
SKIPPED=()
RESULTS=()

FAILED_COUNT=0
TOTAL_ERRORS=0
TOTAL_WARNINGS=0

die() {
    echo "Error: $*" >&2
    exit 1
}

# Detects the host operating system and CPU architecture.
# The resulting values are used to select native or cross-compilation.
detect_host() {
    case "$(uname -s)" in
        Linux)
            HOST_OS="linux"
            ;;
        Darwin)
            HOST_OS="apple"
            ;;
        FreeBSD)
            HOST_OS="freebsd"
            ;;
        OpenBSD)
            HOST_OS="openbsd"
            ;;
        NetBSD)
            HOST_OS="netbsd"
            ;;
        *)
            die "Unsupported host OS: $(uname -s)"
            ;;
    esac

    case "$(uname -m)" in
        x86_64|amd64)
            HOST_ARCH="x86_64"
            ;;
        aarch64|arm64)
            HOST_ARCH="aarch64"
            ;;
        *)
            die "Unsupported host architecture: $(uname -m)"
            ;;
    esac

    echo "Host: $HOST_OS ($HOST_ARCH)"
}

# Determines whether a target can be built in the current environment.
#
# Return values:
#   native - Target matches the host OS and architecture.
#   zig    - Target requires Zig-based cross-compilation.
#   1      - No supported build strategy is available.
#
# Native compilation is preferred whenever the target matches the host.
build_strategy() {
    local target="$1"
    local target_os
    local target_arch

    case "$target" in
        x86_64-unknown-linux-gnu|x86_64-unknown-linux-musl)
            target_os="linux"
            ;;
        x86_64-pc-windows-gnu)
            target_os="windows"
            ;;
        x86_64-apple-darwin|aarch64-apple-darwin)
            target_os="apple"
            ;;
        x86_64-unknown-freebsd)
            target_os="freebsd"
            ;;
        x86_64-unknown-openbsd)
            target_os="openbsd"
            ;;
        x86_64-unknown-netbsd)
            target_os="netbsd"
            ;;
        *)
            return 1
            ;;
    esac

    target_arch="${target%%-*}"

    # Native compilation requires an exact OS and architecture match (only for glibc on standard Linux).
    if [[ "$target" == "x86_64-unknown-linux-gnu" &&
          "$HOST_OS" == "linux" &&
          "$HOST_ARCH" == "x86_64" ]]; then
        echo "native"
        return 0
    fi

    # Zig provides cross-compilation for Windows GNU and Linux musl.
    if [[ "$HOST_OS" == "linux" &&
          ( "$target" == "x86_64-pc-windows-gnu" ||
            "$target" == "x86_64-unknown-linux-musl" ) ]]; then
        echo "zig"
        return 0
    fi

    return 1
}

# Validates the tools required for target installation and source checking.
check_prereqs() {
    command -v cargo >/dev/null 2>&1 \
        || die "cargo not found"

    command -v rustup >/dev/null 2>&1 \
        || die "rustup not found"
}

# Validates additional tools only when at least one selected target
# requires Zig-based cross-compilation.
check_build_prereqs() {
    [[ "$MODE" == "check" ]] && return 0

    local target
    local strategy
    local needs_zig=0

    for target in "${AVAILABLE[@]}"; do
        if strategy=$(build_strategy "$target"); then
            if [[ "$strategy" == "zig" ]]; then
                needs_zig=1
                break
            fi
        fi
    done

    if [[ "$needs_zig" -eq 1 ]]; then
        command -v zig >/dev/null 2>&1 \
            || die "zig not found (needed for cross-compilation)"

        cargo zigbuild --help >/dev/null 2>&1 \
            || die "cargo-zigbuild not found (cargo install cargo-zigbuild)"
    fi
}

# Installs all configured Rust compilation targets.
# Installation failures are recorded without terminating the remaining targets.
install_targets() {
    echo
    echo "=== Installing Rust targets ==="

    for target in "${TARGETS[@]}"; do
        if rustup target add "$target" >/dev/null 2>&1; then
            AVAILABLE+=("$target")
        else
            SKIPPED+=("$target")
            echo "Skipping $target: not available via rustup"
        fi
    done
}

# Filters compiler diagnostics while excluding Cargo summary messages.
# Diagnostic continuation lines are intentionally handled separately.
problem_lines() {
    local pattern="$1"
    local log="$2"

    grep -E "$pattern" "$log" \
        | grep -vE 'could not compile|generated [0-9]+ warning'
}

# Counts matching diagnostic header lines.
# This represents diagnostic entries, not necessarily unique root causes.
count_matches() {
    problem_lines "$1" "$2" | wc -l | tr -d ' '
}

# Executes one compilation or verification operation.
# Each target receives an independent log file.
run_step() {
    local step="$1"
    local target="$2"
    local log="$LOG_DIR/$step-$target.log"

    local status
    local errors
    local warnings
    local start
    local elapsed
    local strategy

    local cmd=()

    case "$step" in
        check)
            cmd=(cargo check --target "$target")
            ;;

        build)
            strategy=$(build_strategy "$target") || return 1

            # Native builds use the platform's default Rust linker.
            # Cross-compilation uses cargo-zigbuild.
            if [[ "$strategy" == "native" ]]; then
                cmd=(cargo build --release --target "$target")
            else
                cmd=(cargo zigbuild --release --target "$target")
            fi
            ;;

        *)
            die "Unknown build step: $step"
            ;;
    esac

    start=$SECONDS

    if "${cmd[@]}" >"$log" 2>&1; then
        status="OK"
    else
        status="FAIL"
        FAILED_COUNT=$((FAILED_COUNT + 1))
    fi

    elapsed=$((SECONDS - start))

    errors=$(count_matches '^error(\[|:)' "$log")
    warnings=$(count_matches '^warning(\[|:)' "$log")

    TOTAL_ERRORS=$((TOTAL_ERRORS + errors))
    TOTAL_WARNINGS=$((TOTAL_WARNINGS + warnings))

    RESULTS+=("$(printf '%-5s %-28s %7s %9s %5ss  %s' \
        "$status" "$target" "$errors" "$warnings" "$elapsed" "$log")")
}

# Prints the current stage results and aggregate diagnostic counts.
print_summary() {
    local step="$1"

    echo
    echo "=== Summary: $step ==="

    printf '%-5s %-28s %7s %9s %6s  %s\n' \
        "STAT" "TARGET" "ERRORS" "WARNINGS" "TIME" "LOG"

    printf '%s\n' "${RESULTS[@]}"

    printf '%-5s %-28s %7s %9s\n' \
        "" "TOTAL" "$TOTAL_ERRORS" "$TOTAL_WARNINGS"
}

# Prints compiler diagnostic headers and their associated source locations.
# Only targets executed during the current stage are inspected.
print_details() {
    local step="$1"
    local target
    local log
    local lines

    echo
    echo "=== Details: $step ==="

    for target in "${AVAILABLE[@]}"; do
        if [[ "$step" == "build" ]]; then
            build_strategy "$target" >/dev/null 2>&1 || continue
        fi

        log="$LOG_DIR/$step-$target.log"

        [[ -f "$log" ]] || continue

        lines=$(problem_lines \
            '^(warning|error)(\[|:)|^ *--> |^ *= note: error:' "$log")

        [[ -z "$lines" ]] && continue

        echo "┌── $target"
        echo "$lines" | sed 's/^/│ /'
        echo "└──"
    done
}

# Executes a complete check or build stage.
# Unsupported build targets are recorded as SKIP instead of FAIL.
run_stage() {
    local step="$1"
    local target
    local strategy

    RESULTS=()
    TOTAL_ERRORS=0
    TOTAL_WARNINGS=0

    mkdir -p "$LOG_DIR"

    echo
    echo "=== Stage: $step ==="

    for target in "${AVAILABLE[@]}"; do
        if [[ "$step" == "build" ]]; then
            if ! strategy=$(build_strategy "$target"); then
                echo "[$step] -> $target (SKIP: unsupported on $HOST_OS/$HOST_ARCH)"

                RESULTS+=("$(printf '%-5s %-28s %7s %9s %6s  %s' \
                    "SKIP" "$target" "-" "-" "-" \
                    "no supported cross-compilation toolchain")")

                continue
            fi

            echo "[$step] -> $target ($strategy)"
        else
            echo "[$step] -> $target"
        fi

        run_step "$step" "$target"
    done

    # Targets that could not be installed are reported separately.
    for target in "${SKIPPED[@]}"; do
        RESULTS+=("$(printf '%-5s %-28s %7s %9s %6s  %s' \
            "SKIP" "$target" "-" "-" "-" "not installed")")
    done

    print_summary "$step"

    if [[ "$SHOW_DETAILS" -eq 1 ]]; then
        print_details "$step"
    fi
}

# Parses arguments, initializes the host environment, and runs requested stages.
main() {
    for arg in "$@"; do
        case "$arg" in
            check|build|all)
                MODE="$arg"
                ;;
            -d|--details)
                SHOW_DETAILS=1
                ;;
            *)
                echo "Usage: $0 [check|build|all] [-d|--details]"
                exit 1
                ;;
        esac
    done

    detect_host
    check_prereqs

    install_targets
    check_build_prereqs

    case "$MODE" in
        check)
            run_stage check
            ;;
        build)
            run_stage build
            ;;
        all)
            run_stage check
            run_stage build
            ;;
    esac

    echo

    if [[ "$FAILED_COUNT" -eq 0 ]]; then
        echo "All attempted targets passed successfully."
    else
        echo "Failed runs: $FAILED_COUNT"
        exit 1
    fi
}

main "$@"
