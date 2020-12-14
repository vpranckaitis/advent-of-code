struct Mask
    mask::UInt64
    bits::UInt64
end

struct Store
    location::UInt
    value::UInt64
end

const Op = Union{Mask, Store}

function read(filename)::Vector{Op}
    return open(filename, "r") do f
        map(eachline(f)) do ln
            m = match(r"^mask = (.*)$", ln)
            if m != nothing
                mask::UInt64 = 0
                bits::UInt64 = 0
                for x in m.captures[1][:]
                    mask <<= 1
                    bits <<= 1
                    x == 'X' && (mask += 1)
                    x == '1' && (bits += 1)
                end
                Mask(mask, bits)
            else
                m = match(r"^mem\[(\d+)\] = (\d+)$", ln)
                Store(parse(UInt, m.captures[1]), parse(UInt64, m.captures[2]))
            end
        end
    end
end

function apply(state, op::Mask)
    return (op, state[2])
end

function apply(state, op::Store)
    mask, mem = state
    mem[op.location] = mask.bits | (mask.mask & op.value)
    (mask, mem)
end

function solve1(ops::Vector{Op})
    state = (Mask(0, 0), Dict{Int, Int}())
    for op in ops
        state = apply(state, op)
    end
    return sum(values(state[2]))
end


function apply2(state, op::Mask)
    return apply(state, op)
end

function apply2(state, op::Store)
    mask, mem = state
    location = (op.location | mask.bits) & ~mask.mask
    bit_pos = Vector{Int}()
    for i = 0:63
        (mask.mask & (1 << i) > 0) && push!(bit_pos, i)
    end
    popcnt = count_ones(mask.mask)
    for a in 0:((1 << popcnt) - 1)
        flocation = location
        for i in 0:(popcnt - 1)
            flocation += ((a >> i) & 1) << bit_pos[i + 1]
        end
        mem[flocation] = op.value
    end
    (mask, mem)
end

function solve2(ops::Vector{Op})
    state = (Mask(0, 0), Dict{Int, Int}())
    for op in ops
        state = apply2(state, op)
    end
    return sum(values(state[2]))
end


function main()
    input = read("files/14.in")
    println(solve1(input))
    println(solve2(input))
end

main()