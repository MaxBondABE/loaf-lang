Loaf: A language for Cellular Automata
--------------------------------------
Loaf is a programming language for expressing Cellular Automata rules.

Here is an example of Conway's Game of Life, written in Loaf.

```
environment := 2D
neighborhood := MOORE

states := {
    Alive::color(black)
    Dead::(default, color(white))
}

rules := {
    from Dead to Alive := neighborhood(Alive) == 3
    from Alive to Dead := neighborhood(Alive) < 2 | neighborhood(Alive) > 3
}
```
