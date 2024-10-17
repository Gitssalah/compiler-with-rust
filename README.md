# compiler-with-rust
<p> Our main objective is to compile a file.lang that contain a simple operation with + - * and / </p>
 <p> example: file.lang </p>
<p>
x = 1; <br>
y = 5; <br>
x + y; <br>
</p>

<p> After compiling we expect the result 6 </p>
 <h3>Step 1 : Lexical analysis</h3>
 <p>According to wikipedia: Lexical analysis is conversion of a text into (semantically or syntactically) meaningful lexical tokens belonging to categorise defined by a "lexer" program </p>

 ![img.png](img.png) 

In this project we will create a list of tokens that will be later converted to assembly

<h3>Step 2 : Parsing  </h3>

