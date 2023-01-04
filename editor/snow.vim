" Vim syntax file
" Language: snow
" Maintainer: Cowboy8625
" Latest Revision: Fri 14 Jan 2022

if exists("b:current_syntax")
  finish
endif

syn keyword snowKeyword if else then
syn keyword snowKeyword true false
syn keyword snowKeyword or and let in
syn keyword snowKeyword fn type enum

syn keyword snowFunction println print

hi link snowKeyword Keyword
" hi link snowFunction Function

" syn match snowFn "\(fn\_s\+\)\@<=\<[A-z0-9]\+\>"
syn match snowFn "^[a-zA-Z][a-zA-z0-9]* "

syn keyword snowTodo contained TODO FIXME XXX NOTE
syn match snowComment "--.*$" contains=snowTodo
" syn match snowCommentBlock  "/{-\%(!\|\*[*/]\@!\)\@!" end="\-}"
syn region snowCommentBlock start="{-\%(!\|\*[*/]\@!\)\@!" end="-}" contains=snowTODO


" Regular int like number with - + or nothing in front
syn match snowNumber '\d\+' contained display
syn match snowNumber '[-+]\d\+' contained display

" Floating point number with decimal no E or e (+,-)
syn match snowNumber '\d\+\.\d*' contained display
syn match snowNumber '[-+]\d\+\.\d*' contained display

" Floating point like number with E and no decimal point (+,-)
syn match snowNumber '[-+]\=\d[[:digit:]]*[eE][\-+]\=\d\+' contained display
syn match snowNumber '\d[[:digit:]]*[eE][\-+]\=\d\+' contained display

" Floating point like number with E and decimal point (+,-)
syn match snowNumber '[-+]\=\d[[:digit:]]*\.\d*[eE][\-+]\=\d\+' contained display
syn match snowNumber '\d[[:digit:]]*\.\d*[eE][\-+]\=\d\+' contained display

syn region snowString start='"' end='"' contained
syn region snowDesc start='"' end='"'

syn match snowHip '\d\{1,6}' nextgroup=snowString
syn region snowDescBlock start="{" end="}" fold transparent contains=ALLBUT,snowHip,crashString

syn keyword snowBlockCmd RA Dec Distance AbsMag nextgroup=snowNumber
syn keyword snowBlockCmd SpectralType nextgroup=snowDesc

syn match snowCharacter /b'\([^\\]\|\\\(.\|x\x\{2}\)\)'/
syn match snowCharacter /'\([^\\]\|\\\(.\|x\x\{2}\|u{\%(\x_*\)\{1,6}}\)\)'/

syn match snowIdentifier contains=snowIdentifierPrime "\%([^[:cntrl:][:space:][:punct:][:digit:]]\|_\)\%([^[:cntrl:][:punct:][:space:]]\|_\)*" display contained


hi def link snowIdentifierPrime   snowIdentifier
hi def link snowIdentifier        Identifier
" hi def link snowFunction          Function
hi def link snowFn                Function
hi def link snowTodo              Todo
hi def link snowComment           Comment
hi def link snowCommentBlock      Comment
hi def link snowBlockCmd          Statement
hi def link snowHip               Type
hi def link snowString            Constant
hi def link snowDesc              PreProc
hi def link snowNumber            Constant
hi def link snowCharacter         Character

let b:current_syntax = "snow"
