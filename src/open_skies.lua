local ffi = require 'ffi'

ffi.cdef [[
void * malloc(size_t);
void free(void *);
]]

local input = io.open("src/pi-billion.txt", "rt")
input:read(2) -- discard "3."


local bitvec = ffi.cast("uint32_t*", ffi.C.malloc(math.ceil(10^11/8)))
local dupcount = 0
local framecount = 0

local buffer = input:read(10000)
while buffer do
  if framecount % 10000 == 0 then
    print("frame", framecount, "dups", dupcount, "time", os.clock())
  end
  framecount = framecount + 1
  for i = 0, #buffer / 10 - 1 do
    local val = tonumber(buffer:sub(i*10+1, i*10+10))
    local offset, shift = bit.rshift(val, 5), bit.band(val, 31)
    local cell = bitvec[offset]
    if bit.band(1, bit.rshift(cell, shift)) == 1 then
      dupcount = dupcount + 1
    else
      bitvec[offset] = bit.bor(cell, bit.lshift(1, shift))
    end
  end
  buffer = input:read(10000)
end

ffi.C.free(bitvec)

print(dupcount)