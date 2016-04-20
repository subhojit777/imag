source $(dirname ${BASH_SOURCE[0]})/../tests/utils.sh

imag-store() {
    imag-call-binary "$(dirname ${BASH_SOURCE[0]})/../target/debug/" imag-store $*
}

