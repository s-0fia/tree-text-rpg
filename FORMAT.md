# Text formatting
## Features
|Feature|Format|Allowed Chars|Escaping|
|---|---|---|---|
|[Text line](#text-lines)|^`> Some text`$|All|N/a|
|[Variables](#variables)|^`>` ... `{var_name}`|a-z, `_` |`{{not a var}}`|
|[Random](#random-selection)|^`>` ... `[op1;option 2;op3]`|All but `;`|`[[Not an option]]`|
|[Styling](#styling)|^`>` ... `STYLE<some text>`|All|`STYLE<<`|
|[Options](#options)|^`[1;2;3;yes]`$|All except `[`, `;`, `]`|N/a|
|[Conditionals](#conditional-functions)|^`{foo(var_name);bar()}`$|function names|N/a|
|[Input](#variables)|^`~var_name=r/.{1,3}/`$|RegEx or function/var names|N/a|

*Note: `^` is start of line and `$` is end of line*

## Text lines


## Variables
To set variables you can use the format of:
* `~foo=bar`, This sets another variable to a new one
* `~foo=baz(bar)`, This applies the function to a variable and sets a var to its result
* `~foo=baz()`, This just runs this function and sets a var to its result
* `~foo~=some value`, This sets the variable to this value
* `~input_var=r/`pattern`/`, This gets input from the user that matches the regex

*Note: for the input regex `^` is prepended and `$` is appended.*

For functions, chaining functions is not possible but can effectively be achieved by:

Ex. `~result=baz(foo(bar))` becomes:
```
~temp=foo(bar)
~result=baz(temp)
```

You may also do a small amount of post process on variables when printing:
|Format|Example|Description|
|---|---|---|
|`{var_name}`|`{age}`→`20`|Plain variable|
|`{U;var_name}`|`{U;name}`→`SOFIA`|Converts to uppercase|
|`{l;var_name}`|`{l;name}`→`sofia`|Converts to lowercase|
|`{F;var_name}`|`{F;name}`→`Sofia`|Converts first character to uppercase|
|`{f;var_name}`|`{f;name}`→`sOFIA`|Converts first character to lowercase|

## Styling
The style options are as follows:
* `BOLD`
* `DIM`
* `ITALIC`
* `UNDERLINED`
* `BLINK`
* `BLINKFAST`
* `REVERSE`
* `HIDDEN`
* `STRIKETHROUGH`

The colouring options are as follows:
* `BLACK`
* `BLUE`
* `GREEN`
* `RED`
* `CYAN`
* `MAGENTA`
* `YELLOW`
* `WHITE`

Each of these colour options may be proceeded by a `B` to make a background colour

These may be composed into something like:
```
> Hello BWHITE<RED<BOLD<traveller>>>
```

## Options
Options 

## Conditional functions

## Random selection

# Tree/file format