#!/usr/bin/env bash
#
# Experimental Protocol Runner
# Runs all experiments defined in the paper appendix
#
# Usage: ./run-experiments.sh [--dry-run] [--experiment NAME]
#
# Experiments: main-grid, ablation, scaling, escalation, difficulty, all
#

set -euo pipefail

# Configuration
LATIN_EXPERIMENT="${LATIN_EXPERIMENT:-./latin-experiment}"
OUTPUT_DIR="${OUTPUT_DIR:-./results}"
TRIALS="${TRIALS:-10}"
DRY_RUN="${DRY_RUN:-false}"

# Model chain for escalation
MODEL_CHAIN="latin-solver,latin-solver-7b,latin-solver-14b"
MODEL_SINGLE="latin-solver"
ESCALATION_THRESHOLD=10

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

run_cmd() {
    local desc="$1"
    shift
    log_info "Running: $desc"
    if [[ "$DRY_RUN" == "true" ]]; then
        echo "  [DRY-RUN] $*"
    else
        "$@"
    fi
}

check_prerequisites() {
    log_info "Checking prerequisites..."

    if [[ ! -x "$LATIN_EXPERIMENT" ]]; then
        log_error "latin-experiment binary not found at: $LATIN_EXPERIMENT"
        exit 1
    fi

    if ! command -v ollama &> /dev/null; then
        log_error "ollama not found in PATH"
        exit 1
    fi

    # Check models are available
    local models=("latin-solver" "latin-solver-7b" "latin-solver-14b")
    for model in "${models[@]}"; do
        if ! ollama list | grep -q "$model"; then
            log_error "Model not found: $model"
            log_error "Create it with: ollama create $model -f Modelfile"
            exit 1
        fi
    done

    log_success "All prerequisites satisfied"
}

setup_output_dir() {
    local timestamp
    timestamp=$(date +%Y%m%d-%H%M%S)
    RESULTS_DIR="${OUTPUT_DIR}/${timestamp}"
    mkdir -p "$RESULTS_DIR"
    log_info "Results will be saved to: $RESULTS_DIR"

    # Save experiment metadata
    cat > "$RESULTS_DIR/metadata.json" <<EOF
{
    "timestamp": "$timestamp",
    "trials": $TRIALS,
    "model_chain": "$MODEL_CHAIN",
    "escalation_threshold": $ESCALATION_THRESHOLD,
    "hostname": "$(hostname)",
    "gpu": "$(nvidia-smi --query-gpu=name --format=csv,noheader 2>/dev/null || echo 'unknown')"
}
EOF
}

# Experiment 1: Main Grid (Strategy Comparison)
# Purpose: Validate that pressure-field coordination outperforms baselines
run_main_grid() {
    log_info "========================================"
    log_info "Experiment 1: Main Grid (Strategy Comparison)"
    log_info "4 strategies × 4 agent counts × $TRIALS trials = $((4 * 4 * TRIALS)) runs"
    log_info "Estimated time: ~45 minutes"
    log_info "========================================"

    run_cmd "Main Grid Experiment" \
        "$LATIN_EXPERIMENT" \
        --model-chain "$MODEL_CHAIN" \
        --escalation-threshold "$ESCALATION_THRESHOLD" \
        grid \
        --trials "$TRIALS" \
        --n 7 \
        --empty 7 \
        --max-ticks 40 \
        --agents 1,2,4,8 \
        --output "$RESULTS_DIR/main-grid.json"

    log_success "Main Grid experiment complete"
}

# Experiment 2: Ablation Study
# Purpose: Validate that each mechanism contributes to performance
# Note: Runs WITHOUT escalation to isolate mechanism effects
run_ablation() {
    log_info "========================================"
    log_info "Experiment 2: Ablation Study"
    log_info "8 configurations × $TRIALS trials = $((8 * TRIALS)) runs"
    log_info "Estimated time: ~20 minutes"
    log_info "========================================"

    run_cmd "Ablation Study" \
        "$LATIN_EXPERIMENT" \
        --model-chain "$MODEL_SINGLE" \
        ablation \
        --trials "$TRIALS" \
        --n 7 \
        --empty 7 \
        --max-ticks 40 \
        --output "$RESULTS_DIR/ablation.json"

    log_success "Ablation study complete"
}

# Experiment 3: Scaling Analysis
# Purpose: Validate Theorem 3 (linear scaling)
run_scaling() {
    log_info "========================================"
    log_info "Experiment 3: Scaling Analysis"
    log_info "6 agent counts × $TRIALS trials = $((6 * TRIALS)) runs"
    log_info "Estimated time: ~30 minutes"
    log_info "========================================"

    run_cmd "Scaling Analysis" \
        "$LATIN_EXPERIMENT" \
        --model-chain "$MODEL_CHAIN" \
        --escalation-threshold "$ESCALATION_THRESHOLD" \
        grid \
        --trials "$TRIALS" \
        --n 7 \
        --empty 8 \
        --max-ticks 40 \
        --agents 1,2,4,8,16,32 \
        --output "$RESULTS_DIR/scaling.json"

    log_success "Scaling analysis complete"
}

# Experiment 4: Model Escalation Impact
# Purpose: Validate that model escalation improves solve rate
run_escalation() {
    log_info "========================================"
    log_info "Experiment 4: Model Escalation Impact"
    log_info "2 configurations × 3 agent counts × $TRIALS trials = $((2 * 3 * TRIALS)) runs"
    log_info "Estimated time: ~30 minutes"
    log_info "========================================"

    # Without escalation
    log_info "Running WITHOUT escalation..."
    run_cmd "Escalation: Single Model" \
        "$LATIN_EXPERIMENT" \
        --model-chain "$MODEL_SINGLE" \
        grid \
        --trials "$TRIALS" \
        --n 7 \
        --empty 8 \
        --max-ticks 40 \
        --agents 2,4,8 \
        --output "$RESULTS_DIR/escalation-without.json"

    # With escalation
    log_info "Running WITH escalation..."
    run_cmd "Escalation: Model Chain" \
        "$LATIN_EXPERIMENT" \
        --model-chain "$MODEL_CHAIN" \
        --escalation-threshold "$ESCALATION_THRESHOLD" \
        grid \
        --trials "$TRIALS" \
        --n 7 \
        --empty 8 \
        --max-ticks 40 \
        --agents 2,4,8 \
        --output "$RESULTS_DIR/escalation-with.json"

    log_success "Escalation impact experiment complete"
}

# Experiment 5: Difficulty Scaling
# Purpose: Show framework handles increasing difficulty
run_difficulty() {
    log_info "========================================"
    log_info "Experiment 5: Difficulty Scaling"
    log_info "4 difficulty levels × $TRIALS trials = $((4 * TRIALS)) runs"
    log_info "Estimated time: ~40 minutes"
    log_info "========================================"

    # Easy: 5x5, 5 empty (20%)
    log_info "Running Easy difficulty (5x5, 5 empty)..."
    run_cmd "Difficulty: Easy" \
        "$LATIN_EXPERIMENT" \
        --model-chain "$MODEL_CHAIN" \
        --escalation-threshold "$ESCALATION_THRESHOLD" \
        grid \
        --trials "$TRIALS" \
        --n 5 \
        --empty 5 \
        --max-ticks 50 \
        --agents 4 \
        --output "$RESULTS_DIR/difficulty-easy.json"

    # Medium: 6x6, 8 empty (22%)
    log_info "Running Medium difficulty (6x6, 8 empty)..."
    run_cmd "Difficulty: Medium" \
        "$LATIN_EXPERIMENT" \
        --model-chain "$MODEL_CHAIN" \
        --escalation-threshold "$ESCALATION_THRESHOLD" \
        grid \
        --trials "$TRIALS" \
        --n 6 \
        --empty 8 \
        --max-ticks 50 \
        --agents 4 \
        --output "$RESULTS_DIR/difficulty-medium.json"

    # Hard: 7x7, 10 empty (20%)
    log_info "Running Hard difficulty (7x7, 10 empty)..."
    run_cmd "Difficulty: Hard" \
        "$LATIN_EXPERIMENT" \
        --model-chain "$MODEL_CHAIN" \
        --escalation-threshold "$ESCALATION_THRESHOLD" \
        grid \
        --trials "$TRIALS" \
        --n 7 \
        --empty 10 \
        --max-ticks 50 \
        --agents 4 \
        --output "$RESULTS_DIR/difficulty-hard.json"

    # Very Hard: 8x8, 14 empty (22%)
    log_info "Running Very Hard difficulty (8x8, 14 empty)..."
    run_cmd "Difficulty: Very Hard" \
        "$LATIN_EXPERIMENT" \
        --model-chain "$MODEL_CHAIN" \
        --escalation-threshold "$ESCALATION_THRESHOLD" \
        grid \
        --trials "$TRIALS" \
        --n 8 \
        --empty 14 \
        --max-ticks 50 \
        --agents 4 \
        --output "$RESULTS_DIR/difficulty-veryhard.json"

    log_success "Difficulty scaling experiment complete"
}

run_all() {
    local start_time
    start_time=$(date +%s)

    log_info "========================================"
    log_info "Running FULL Experimental Protocol"
    log_info "Total estimated time: ~3 hours"
    log_info "========================================"

    run_main_grid
    run_ablation
    run_scaling
    run_escalation
    run_difficulty

    local end_time
    end_time=$(date +%s)
    local duration=$((end_time - start_time))
    local hours=$((duration / 3600))
    local minutes=$(((duration % 3600) / 60))

    log_success "========================================"
    log_success "All experiments complete!"
    log_success "Total runtime: ${hours}h ${minutes}m"
    log_success "Results saved to: $RESULTS_DIR"
    log_success "========================================"
}

print_usage() {
    cat <<EOF
Usage: $0 [OPTIONS] [EXPERIMENT]

Options:
    --dry-run       Print commands without executing
    --trials N      Number of trials per config (default: 10)
    --output DIR    Output directory (default: ./results)
    -h, --help      Show this help message

Experiments:
    main-grid       Strategy comparison (4 strategies × 4 agent counts)
    ablation        Mechanism ablation study (8 configurations)
    scaling         Agent scaling analysis (1-32 agents)
    escalation      Model escalation impact comparison
    difficulty      Difficulty scaling (5x5 to 8x8)
    all             Run all experiments (default)

Examples:
    $0                      # Run all experiments
    $0 main-grid            # Run only main grid experiment
    $0 --dry-run all        # Preview all commands
    $0 --trials 5 ablation  # Run ablation with 5 trials
EOF
}

main() {
    local experiment="all"

    while [[ $# -gt 0 ]]; do
        case $1 in
            --dry-run)
                DRY_RUN="true"
                shift
                ;;
            --trials)
                TRIALS="$2"
                shift 2
                ;;
            --output)
                OUTPUT_DIR="$2"
                shift 2
                ;;
            -h|--help)
                print_usage
                exit 0
                ;;
            main-grid|ablation|scaling|escalation|difficulty|all)
                experiment="$1"
                shift
                ;;
            *)
                log_error "Unknown option: $1"
                print_usage
                exit 1
                ;;
        esac
    done

    check_prerequisites
    setup_output_dir

    case $experiment in
        main-grid)
            run_main_grid
            ;;
        ablation)
            run_ablation
            ;;
        scaling)
            run_scaling
            ;;
        escalation)
            run_escalation
            ;;
        difficulty)
            run_difficulty
            ;;
        all)
            run_all
            ;;
    esac
}

main "$@"
