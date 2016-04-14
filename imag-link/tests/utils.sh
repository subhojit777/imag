source $(dirname ${BASH_SOURCE[0]})/../../tests/utils.sh

imag-link() {
    imag-call-binary "$(dirname ${BASH_SOURCE[0]})/../target/debug/" imag-link $*
}

