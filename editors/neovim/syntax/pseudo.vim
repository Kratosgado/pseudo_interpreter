if exists("b:current_syntax")
  finish
endif

syn case ignore

" Control flow
syn keyword pseudoConditional if elseif else endif then
syn keyword pseudoRepeat      while endwhile for endfor to step do
syn keyword pseudoStatement   return break continue

" Functions
syn keyword pseudoFunction    function endfunction

" Declarations
syn keyword pseudoKeyword     declare as

" Built-in I/O
syn keyword pseudoBuiltin     print output display input get receive

" Types
syn keyword pseudoType        integer double string boolean

" Boolean literals
syn keyword pseudoBoolean     true false

" Logical operators
syn keyword pseudoOperator    and or not

" Numbers
syn match pseudoNumber '\b\d\+\b'
syn match pseudoFloat  '\b\d\+\.\d\+\b'

" Strings
syn region pseudoString  start='"' end='"' contains=pseudoEscape
syn match  pseudoEscape  '\\.' contained

" Comments (# to end of line)
syn match pseudoComment '#.*$'

" Function calls (identifier followed by open paren)
syn match pseudoFuncCall '\b[a-zA-Z_][a-zA-Z0-9_]*\b\ze\s*('

" Comparison / assignment operators
syn match pseudoOperatorSym '[<>]=\?'
syn match pseudoOperatorSym '=='
syn match pseudoOperatorSym '!='

hi def link pseudoConditional  Conditional
hi def link pseudoRepeat       Repeat
hi def link pseudoStatement    Statement
hi def link pseudoFunction     Keyword
hi def link pseudoKeyword      Keyword
hi def link pseudoBuiltin      Function
hi def link pseudoType         Type
hi def link pseudoBoolean      Boolean
hi def link pseudoNumber       Number
hi def link pseudoFloat        Float
hi def link pseudoString       String
hi def link pseudoEscape       SpecialChar
hi def link pseudoComment      Comment
hi def link pseudoOperator     Keyword
hi def link pseudoOperatorSym  Operator
hi def link pseudoFuncCall     Function

let b:current_syntax = "pseudo"
