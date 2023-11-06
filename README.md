# Drevne_Ruso++


## Syntax
```
ВОДА_БАЙКАЛА() - outputs a variable
ЦАРЬ_БАТЮШКА() - Displays text in brackets (use _ instead of spaces)
```

## Example
```
ящер = 10 / 2;
ВОДА_БАЙКАЛА(ящер);
ящер = ящер + 10 - 12 + 100;
ВОДА_БАЙКАЛА(ящер);
ящер = ящер * 3 + ( 2 * 3 );
ВОДА_БАЙКАЛА(ящер);
славный_молодец = 20 / 10 - ( 30 * 20 );
ВОДА_БАЙКАЛА(славный_молодец);
ЦАРЬ_БАТЮШКА(Крутой);
ящер = 5 ^ 2;
ВОДА_БАЙКАЛА(ящер);
ящер = √ящер;
ВОДА_БАЙКАЛА(ящер);

ЦАРЬ_БАТЮШКА(Язык_разработан_Elieren);
```

## Output in terminal
```
5
103
315
-598
Крутой
25
5
Язык_разработан_Elieren
```

## Commands to create a compiler file
```
cargo build --release
```

## Compiling Code
```
app.exe {Path to the code file}
```

## Fixed bugs

- [x] Bug with calculation of mathematical problems. In some examples it may give an incorrect answer.

## Features

- [x] Mathematical operations
- [x] Variables
- [ ] Working with strings (addition, slicing)
- [ ] Conditional statement (if-else)

That's all for now