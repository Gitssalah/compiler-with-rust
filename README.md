# compiler-with-rust
<p> Our main objective is to compile a file.lang that contain a simple operation with + - * and / </p>
 <p> example: file.lang </p>
<p>
x = 1; <br>
y = 5; <br>
print(x + y); <br>
</p>

<p> After compiling we expect the result 6 </p>
 <h3>Step 1 : Lexical analysis</h3>
 <p>According to wikipedia: Lexical analysis is conversion of a text into (semantically or syntactically) meaningful lexical tokens belonging to categorise defined by a "lexer" program </p>

 ![img.png](img.png) 

In this project we will create a list of tokens that will be later converted to assembly

<h3>Step 2 : Syntax Analysis (Parsing)  </h3>
<p>Parsing is the next phase after tokenization. It involves taking the stream of tokens from the lexer and analyzing how they fit together according to the grammar rules of the language.</p>
<p>The output of this phase is usually an Abstract Syntax Tree (AST), a hierarchical representation of the structure of the program.</p>

<h3> Step 3 : Error Handling</h3>
 <p>We will add an error handler to show print error type and a message explaining the error</p>
 <p>Todo: add diagram of error handing </p>
