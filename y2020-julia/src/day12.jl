const Command = Tuple{Char, Int}
const Position = Array{Int}
const State = Tuple{Position, Position}

function read(filename)::Vector{Command}
    return open(filename, "r") do f
        return map(eachline(f)) do ln
            return (ln[1], parse(Int, ln[2:end]))
        end
    end
end

const directions = Dict(
    'N' => [0, 1],
    'S' => [0, -1],
    'E' => [1, 0],
    'W' => [-1, 0],
)

function apply(state, command, move_point)
    t, cnt = command
    return if in(t, ['L', 'R'])
        dir = state[2]
        for i in 1:div(cnt, 90)
            dir = [dir[2], -dir[1]]
            if t == 'L'
                dir *= -1
            end
        end
        (state[1], dir)
    elseif t == 'F'
        dir = state[2]
        (state[1] + dir * cnt, dir)
    elseif move_point
        (state[1], state[2] + directions[t] * cnt)
    else
        (state[1] + directions[t] * cnt, state[2])
    end
end

function solve1(data)
    state = ([0, 0], [1, 0])
    for c in data
        state = apply(state, c, false)
    end

    return abs(state[1][1]) + abs(state[1][2])
end

function solve2(data)
    state = ([0, 0], [10, 1])
    for c in data
        state = apply(state, c, true)
    end

    return abs(state[1][1]) + abs(state[1][2])
end


function main()
    input = read("files/12.in")
    println(solve1(input))
    println(solve2(input))
end

main()