## Snake

Snake is a typescript-like programming language that compiles to [brainf*ck](https://esolangs.org/wiki/Brainfuck), an
esoteric language.

---

Example:

```ts
let A = 20;
let B = sum(A, A);
print(sum(A, B, 2), sum(A, A, A));
```

A is 20

B is A + A = 40

A + B + 2 = 62 = '>'

A + A + A = 60 = '<'

should print `> <` with a newline.

This code compiles to:

```bf
++++++++++++++++++++><[>+>+<<-]>>[<<+>>-]<<[>>+>+<<<-]>>>
[<<<+>>>-]<[->+<]><<[->>+<<]>>><<<[-]>>[<<+>>-]><[-]<[-]<
<[>>+>+<<<-]>>>[<<<+>>>-]<<<[>>>+>+<<<<-]>>>>[<<<<+>>>>-]
<<<<[>>>>+>+<<<<<-]>>>>>[<<<<<+>>>>>-]<[->+<]><<[->>+<<]>
><<<[->>>+<<<]>>>><<<<[-]>>>[<<<+>>>-]><[-]<[-]<[-]++><<<
[>>>+>+<<<<-]>>>>[<<<<+>>>>-]<<<<<[>>>>>+>+<<<<<<-]>>>>>>
[<<<<<<+>>>>>>-]<[->+<]><<[->>+<<]>><<<[->>>+<<<]>>>><<<<
[-]>>>[<<<+>>>-]><[-]<[-]<[-]>++++[<++++++++>-]<<.>.<<.>>
>+++[<------->-]<-.[-]<<[-]>>[<<+>>-]<[-]
```

prints `> <`!

---

Snake current supports:

- Variable assignment:

```ts
let myVariable = 2;
```

- Print to console (I am too lazy to implement console.log)

```ts
print(1, 2, x)
```

- Types: Byte, Unit, Function

- Sum of bytes:

```ts
print(sum(60, 2, 3))
```
prints 'A'
