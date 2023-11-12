# Drevne_Ruso++


## Syntax
```
+ - Plus
- - Minus
* - Multiply
/ - Divide
^ - Exponentiation
√ - Sqrt
"" - Straing
ВОДА_БАЙКАЛА() - outputs a variable
ЦАРЬ_БАТЮШКА() - Displays text in brackets (use _ instead of spaces)
```

## Example
```
добрыня = "муромец";
ВОДА_БАЙКАЛА(добрыня);
добрыня = "муромец" + "_" + "красава";
ВОДА_БАЙКАЛА(добрыня);
добрыня = 10;
ВОДА_БАЙКАЛА(добрыня);
добрыня = "муромец";
ВОДА_БАЙКАЛА(добрыня);
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
ящер = √( ящер );
ВОДА_БАЙКАЛА(ящер);
слава_руси = ящер;
ВОДА_БАЙКАЛА(слава_руси);

ЦАРЬ_БАТЮШКА(Язык_разработан_Elieren);
```

## Output in terminal
```
муромец
муромец_красава
10
муромец
5
103
315
-598
Крутой
25
5
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
- [ ] Working with strings ( [x] addition, [ ] slicing)
- [ ] Conditional statement (if-else)

That's all for now
