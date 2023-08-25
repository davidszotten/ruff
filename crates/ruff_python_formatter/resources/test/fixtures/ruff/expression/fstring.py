(
    f'{one}'
    f'{two}'
)

# quote handling for raw f-strings
rf"Not-so-tricky \"quote"

# Regression test for fstrings dropping comments
result_f = (
    'Traceback (most recent call last):\n'
    f'  File "{__file__}", line {lineno_f+5}, in _check_recursive_traceback_display\n'
    '    f()\n'
    f'  File "{__file__}", line {lineno_f+1}, in f\n'
    '    f()\n'
    f'  File "{__file__}", line {lineno_f+1}, in f\n'
    '    f()\n'
    f'  File "{__file__}", line {lineno_f+1}, in f\n'
    '    f()\n'
    # XXX: The following line changes depending on whether the tests
    # are run through the interactive interpreter or with -m
    # It also varies depending on the platform (stack size)
    # Fortunately, we don't care about exactness here, so we use regex
    r'  \[Previous line repeated (\d+) more times\]' '\n'
    'RecursionError: maximum recursion depth exceeded\n'
)


# Regression for fstring dropping comments that were accidentally attached to
# an expression inside a formatted value
(
    f'{1}'
    # comment
    ''
)


f"{ chr(65)  =   }"
f"{ chr(65)  =   !s}"
f"{ chr(65)  =   !r}"
f"{ chr(65)  =   :#x}"
f"{a=!r:0.05f}"

f"{ {}  =   }"
f"{ {}=}"

# should add some nice spaces
f"{1-2+3}"


# don't switch quotes inside a formatted value
f"\"{f'{nested} inner'}\" outer"

# need a space to avoid escaping the curly
f"{ {1}}"

# extra spaces don't interfere with debug_text
f"{ {1}=}"

# handle string continuations
(
    ''
    f'"{1}'
)

# it's ok to change triple quotes with even with qoutes inside f-strings
f''' {""} '''


# various nested quotes
f' {f" {1}"} '
f" {f' {1}'} "
