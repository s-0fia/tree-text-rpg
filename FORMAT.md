# Features
|Feature|Format|Allowed Chars|Escaping|
|---|---|---|---|
|[Text line](#text-lines)|^`> Some text`$|All|N/a|
|[Variables](#variables)|^`>` ... `{var_name}`|a-z, `_` |`{{not a var}}`|
|[Random](#random-selection)|^`>` ... `[op1;option 2;op3]`|All but `;`|`[[Not an option]]`|
|[Styling](#styling)|^`>` ... `STYLE<some text>`|All|`STYLE<<`|
|[Options](#options)|^`[1;2;3;yes]`$|All except `[`, `;`, `]`|N/a|
|[Conditionals](#conditional-functions)|^`{foo()}`$|function names|N/a|
|[Input](#variables)|^`~var_name=r/.{1,3}/`$|RegEx or function/var names|N/a|

*Note: `^` is start of line and `$` is end of line.*

# Text lines
Text lines print to the screen, in them you can insert [variables](#variables), [styling](#styling), and [random selection](#random-selection).
An example of this is:
```
> This is just a plain text line
> Or you can use an input, {name}!
> Maybe you want to BWHITE<BLUE<BOLD<style>>> something.
> Maybe you want a [cool ;super sexy ;]random text.
> You can still use {{ and }} like this, or
> for some reason write BLUE<< like this.
> [[Sofia]] These can be used too!
```

# Variables
To set variables you can use the format of:
* `~foo=bar`, This sets another variable to a new one
* `~foo=baz()`, This just runs this function and sets a var to its result
* `~foo~=some value`, This sets the variable to this value
* `~input_var=r/`pattern`/`, This gets input from the user that matches the regex

*Note: for the input regex `^` is prepended and `$` is appended.*

For functions, chaining functions is not possible but can effectively be achieved by:

Ex. `~result=baz(foo(bar()))` becomes:
```
~temp=bar()
~temp=foo()
~result=baz()
```

Where the lua would look something like: 
```lua
local function bar()
  return "something"
end

local function foo()
  local bar = get_var("temp")
  -- ...
  return processed_bar
end

local function baz()
  local foo = get_var("temp")
  -- ...
  return processed_foo
end
```

You may also do a small amount of post process on variables when printing:
|Format|Example|Description|
|---|---|---|
|`{var_name}`|`{age}`→`20`|Plain variable|
|`{U;var_name}`|`{U;name}`→`SOFIA`|Converts to uppercase|
|`{l;var_name}`|`{l;name}`→`sofia`|Converts to lowercase|
|`{F;var_name}`|`{F;name}`→`Sofia`|Converts first character to uppercase|
|`{f;var_name}`|`{f;name}`→`sOFIA`|Converts first character to lowercase|

# Styling
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

# Options
Options are for selecting the next node to go to.
The position of input selected will select the next node to traverse to.

Example:
```
> Where do you wish to go traveller?
> Left towards the village, ahead towards the forest, right towards the beach?
[left;ahead;right]
```
```
[left;ahead;right]→[1;2;3]
        O
       /|\
      / | \
left /  |  \ right 
    1   2   3
      ahead
```

# Conditional functions
Conditionals are for choosing a node/path based on conditional logic on game/player state etc.
The conditionals' result should be a positive integer to determine the next node to traverse to (0 indexed).
The conditional functions are written in lua.

*Note: conditionals are case-insensitive.*

Example:
```
{health_check()}
```
lua:
```lua
local function health_check()
  local health = tonumber(get_var("health"))
  if health < 50 then
    return 0
  else
    return 1
  end
end
```
```
          O
         / \
health  /   \  health
 <50   0     1   ≥50
```

# Random selection
This is for random text insertion for text lines, this is for simple equal distribution randomness.
For more complex distributions use a lua function to set a variable and use the variable in the text.

Example:
```
> You can use this feature to [make random text;cause chaos;do random things].
```
