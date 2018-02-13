# Documentation for the parser
## Type of parser
This parser is a extended Shunting-Yard algorithm. Information used in construction of this parser
can be found (here) [http://wcipeg.com/wiki/Shunting_yard_algorithm#Unary_operators]
## Pseudocode
```
operator_stack = {}
tree_stack = {}
while t in tokens:
	if t == operand:
		tree_stack.push(t)
	if t == operator:
		while operator_stack.first has higher or equal precedence
			op = operator_stack.pop
			tree_stack.push(Node(op, tree_stack.pop, tree_stack.pop))
		operator_stack.push(t)
	if t == '(':
		operator_stack.push(t)
	if t == ')':
		while op in operator_stack is not '(':
			tree_stack.push(Node(op, tree_stack.pop, tree_stack.pop))
	
while op in operator_stack:
	tree_stack.push(Node(op, tree_stack.pop, tree_stack.pop))

```
