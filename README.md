qcalc
==========

qcalc is a simple and effective calculator with a terminal user interface. It serves to avoid
having to open up a web browser, phone or desktop calculator application for those who work from
the terminal

### Usage
- Install cargo (instructions [here](https://doc.rust-lang.org/cargo/getting-started/installation.html))
- Install the binary `cargo install qcalc`
- Run with `qcalc`
- Enjoy!

## Disclaimer of Warranty
The software is provided "as is," without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose, and noninfringement. In no event shall the authors or copyright holders be liable for any claim, damages, or other liability, whether in an action of contract, tort, or otherwise, arising from, out of, or in connection with the software or the use or other dealings in the software.

### Features
- Functions as types `let pow = |a, b| a ** b` `map([1, 2, 3], |x| x ** 3)`
- Copy paste
- Saves expressions which you can re-populate your input field with
- Ability to save calculation results in variables
- Built in functions
- Resetting variables
- Binary and hexadecimal inputs and bitwise operations eg. "0xff + 0b10 / 10", "0b1000001 ^ 0b100"
- Tab completions

#### Feature requests / Bug reports
- Feel free to open an issue and i'll look into it
