" Title:        Convert
" Description:  A plugin that provides a set of commands to convert text to
" whatever
" Last Change:  2024-07-02
" Maintainer:   Rubenduburck <github.com/rubenduburck>

" Avoid loading twice
if exists('g:loaded_convert')
    finish
endif
let g:loaded_convert = 1

if !exists('s:convertJobId')
    let s:convertJobId = 0
endif

function! s:get_visual_selection()
    " Why is this not a built-in Vim script function?!
    let [line_start, column_start] = getpos("'<")[1:2]
    let [line_end, column_end] = getpos("'>")[1:2]
    let lines = getline(line_start, line_end)
    if len(lines) == 0
        return ''
    endif
    let lines[-1] = lines[-1][: column_end - (&selection == 'inclusive' ? 1 : 2)]
    let lines[0] = lines[0][column_start - 1:]
    return join(lines, "\n")
endfunction

" Constants for RPC messages
let s:Alch = 'classify_and_convert'
let s:AlchFlatten = 'flatten_array'
let s:AlchChunk = 'chunk_array'
let s:AlchReverse = 'reverse_array'
let s:AlchRotate = 'rotate_array'
let s:AlchGenerate = 'generate'
let s:AlchRandom = 'random'
let s:AlchPadLeft = 'pad_left'
let s:AlchPadRight = 'pad_right'
let s:AlchStart = 'start'
let s:AlchStop = 'stop'

let s:script_dir = expand('<sfile>:p:h')
let s:bin = s:script_dir . '/../target/release/vim-alchemy'

"Entry point
function! s:connect()
    let id = s:initRpc()

    if id == 0
        echoerr "convert: cannot start rpc process"
    elseif id == -1
        echoerr "convert: rpc process is not executable"
    else
        let s:convertJobId = id

        call s:configureCommands()
    endif
endfunction

function! s:configureCommands()
    command! -range -nargs=+ Alch :call s:classify_and_convert(<f-args>)
    command! -range AlchFlatten :call s:flatten_array()
    command! -range -nargs=+ AlchChunk :call s:chunk_array(<f-args>)
    command! -range -nargs=* AlchReverse :call s:reverse_array()
    command! -range -nargs=+ AlchRotate :call s:rotate_array(<f-args>)
    command! -nargs=+ AlchGenerate :call s:generate(<f-args>)
    command! -nargs=+ AlchRandom :call s:random(<f-args>)
    command! -range -nargs=+ AlchPadLeft :call s:pad_left(<f-args>)
    command! -range -nargs=+ AlchPadRight :call s:pad_right(<f-args>)

    command! AlchStart :call s:start()
    command! AlchStop :call s:stop()
endfunction

function! s:classify_and_convert(...)
    let encoding = get(a:,1, 'int')
    let input = s:get_visual_selection()
    call rpcnotify(s:convertJobId, s:Alch, encoding, input)
endfunction

function! s:flatten_array(...)
    let input = s:get_visual_selection()
    call rpcnotify(s:convertJobId, s:AlchFlatten, input)
endfunction

function! s:chunk_array(...)
    let chunk_count = get(a:,1, 'int')
    let input = s:get_visual_selection()
    call rpcnotify(s:convertJobId, s:AlchChunk, chunk_count, input)
endfunction

function! s:reverse_array()
    let depth = get(a:,1, '1')
    let input = s:get_visual_selection()
    call rpcnotify(s:convertJobId, s:AlchReverse, depth, input)
endfunction

function! s:rotate_array(...)
    let rotation = get(a:,1, '1')
    let input = s:get_visual_selection()
    call rpcnotify(s:convertJobId, s:AlchRotate, rotation, input)
endfunction

function! s:generate(...)
    let encoding = get(a:,1, 'int')
    let bytes = get(a:,2, 'int')
    call rpcnotify(s:convertJobId, s:AlchGenerate, encoding, bytes)
endfunction

function! s:random(...)
    let encoding = get(a:,1, 'int')
    let bytes = get(a:,2, 'int')
    call rpcnotify(s:convertJobId, s:AlchRandom, encoding, bytes)
endfunction

function! s:pad_left(...)
    let padding = get(a:,1, ' ')
    let input = s:get_visual_selection()
    call rpcnotify(s:convertJobId, s:AlchPadLeft, padding, input)
endfunction

function! s:pad_right(...)
    let padding = get(a:,1, ' ')
    let input = s:get_visual_selection()
    call rpcnotify(s:convertJobId, s:AlchPadRight, padding, input)
endfunction

function! s:start()
    let id = s:initRpc()
    let s:convertJobId = id
endfunction

function! s:stop()
    call rpcnotify(s:convertJobId, s:AlchStop)
    let s:convertJobId = 0
endfunction

function! s:initRpc()
    if s:convertJobId == 0
        let jobid = jobstart([s:bin], {'rpc': v:true})
        return jobid
    else
        return s:convertJobId
    endif
endfunction

call s:connect()
