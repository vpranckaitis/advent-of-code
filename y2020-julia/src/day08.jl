struct Nop
    x::Int
end

struct Acc
    x::Int
end

struct Jmp
    x::Int
end

const Op = Union{Nop, Acc, Jmp}

const State = Tuple{Int, Int}

function read(filename)::Vector{Op}
    return open(filename, "r") do f
        return map(eachline(f)) do ln
            parts = split(ln, " ")

            if parts[1] == "nop"
                op = Nop
            elseif parts[1] == "acc"
                op = Acc
            elseif parts[1] == "jmp"
                op = Jmp
            else
                error("unknown command")
            end
            return op(parse(Int, parts[2]))
        end
    end
end

apply(state::State, nop::Nop) = (state[1] + 1, state[2])
apply(state::State, acc::Acc) = (state[1] + 1, state[2] + acc.x)
apply(state::State, jmp::Jmp) = (state[1] + jmp.x, state[2])

function run(ops::Vector{Op})::State
    positions = Set{Int}()
    state = (1, 0)

    while !in(state[1], positions) && state[1] <= length(ops)
        push!(positions, state[1])
        state = apply(state, ops[state[1]])
    end

    return state
end

function solve1(ops::Vector{Op})
    return run(ops)[2]
end

swapnopjmp(op::Nop) = Jmp(op.x)
swapnopjmp(op::Jmp) = Nop(op.x)
swapnopjmp(op::Acc) = op

function solve2(ops::Vector{Op})
    for i in 1:length(ops)
        op1 = ops[i]
        op2 = swapnopjmp(op1)
        if op1 != op2
            ops[i] = op2
            state = run(ops)
            ops[i] = op1

            state[1] > length(ops) && return state[2]
        end
    end
    error("")
end


function main()
    input = read("files/08.in")
    println(solve1(input))
    println(solve2(input))
end

main()