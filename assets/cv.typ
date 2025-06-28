#set page(paper:"a4", margin: (x: 8pt, y: 8pt), fill: none)
#let myblock = block.with(inset: 1em)

// #block(inset:1em)[
#myblock[
#set text(bottom-edge: "descender")
= First title
#lorem(15)
]

#myblock[
= Second title
#lorem(20)

== Subtitle of the second part
#lorem(5)

== Another subtitle of the second part
#lorem(5)
]

#myblock[= Third title
#lorem(15)
]

#myblock[
= Fourth title
#lorem(10)\
#lorem(3)
]

#myblock[
= Fifth title
#lorem(15)
]

#myblock[
= Sixth title
#lorem(15)
]
