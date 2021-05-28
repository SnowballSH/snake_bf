## Snake

Snake is a typescript-like programming language that compiles to [brainf*ck](https://esolangs.org/wiki/Brainfuck), an
esoteric language.

---

Example:

```ts
let thirteen = 13;
let A = thirteen + thirteen - 6;
let B = A + A;
print(sum(A, B, 2), sum(A, A, A));
```

thirteen is 13

A is 13 + 13 - 6 = 20

B is A + A = 40

A + B + 2 = 62 = '>'

A + A + A = 60 = '<'

should print `> <` with a newline.

This code compiles to:

```bf
+++++++++++++><[>+>+<<-]>>[<<+>>-]<<[>>+>+<<<-]
>>>[<<<+>>>-]<[->+<]<[->>+<<]>>><<<[-]>>[<<+>>-
]><[-]<[-]++++++><<[->>+<<]>[->-<]>><<<[-]>>[<<
+>>-]><[-]<[-]<[>+>+<<-]>>[<<+>>-]<<[>>+>+<<<-]
>>>[<<<+>>>-]<[->+<]<[->>+<<]>>><<<[-]>>[<<+>>-
]><[-]<[-]<<[>>+>+<<<-]>>>[<<<+>>>-]<<<[>>>+>+<
<<<-]>>>>[<<<<+>>>>-]<<<<[>>>>+>+<<<<<-]>>>>>[<
<<<<+>>>>>-]<[->+<]><<[->>+<<]>><<<[->>>+<<<]>>
>><<<<[-]>>>[<<<+>>>-]><[-]<[-]<[-]++><<<[>>>+>
+<<<<-]>>>>[<<<<+>>>>-]<<<<<[>>>>>+>+<<<<<<-]>>
>>>>[<<<<<<+>>>>>>-]<[->+<]><<[->>+<<]>><<<[->>
>+<<<]>>>><<<<[-]>>>[<<<+>>>-]><[-]<[-]<[-]>+++
+[<++++++++>-]<<.>.<<.>>>+++[<------->-]<-.[-]<
<[-]>>[<<+>>-]<[-]
```

prints `> <`!

---

Snake current supports:

- Variable assignment:

```ts
let myVariable = 2;
```

- Binary operations:

```ts
1 + 2 - 4 * 5 / 6 % 7
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
