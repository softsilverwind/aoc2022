require 'json'

def cmp this, that
    if this.class == Integer && that.class == Integer
        if this < that
            -1
        elsif this == that
            0
        else
            1
        end
    elsif this.class == Integer && that.class == Array
        cmp [this], that
    elsif this.class == Array && that.class == Integer
        cmp this, [that]
    elsif this.length == 0 && that.length > 0
        -1
    elsif this.length > 0 && that.length == 0
        1
    elsif this.length == 0 && that.length == 0
        0
    else
        first = cmp this[0], that[0]
        if first != 0
            first
        else
            cmp this[1..-1], that[1..-1]
        end
    end
end

def main_simple
    pairs = []

    loop do
        line1 = STDIN.gets
        break if line1.nil?
        packet1 = JSON.parse line1
        line2 = STDIN.gets
        packet2 = JSON.parse line2
        pairs << [packet1, packet2]
        STDIN.gets
    end

    res = []

    for pair, index in pairs.each_with_index
        if cmp(pair[0], pair[1]) == -1
            res << index + 1
        end
    end

    puts res.sum
end

def main_complex
    packets = []

    loop do
        line1 = STDIN.gets
        break if line1.nil?
        packets << JSON.parse(line1)
        line2 = STDIN.gets
        packets << JSON.parse(line2)
        STDIN.gets
    end

    packets << [[2]]
    packets << [[6]]

    packets.sort! { |a, b| cmp a, b }
    i1 = packets.find_index { |x| x == [[2]] } + 1
    i2 = packets.find_index { |x| x == [[6]] } + 1
    puts i1 * i2
end

main_complex
