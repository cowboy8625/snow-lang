def get_bytes(filename):
    with open(filename, 'rb') as f:
        return f.read()

def get_section_name(id: int) -> str:
    match id:
        case 1:
            return "Type\n"
        case 2:
            return "Import\n"
        case 3:
            return "Function\n"
        case 4:
            return "Table\n"
        case 5:
            return "Memory\n"
        case 6:
            return "Global\n"
        case 7:
            return "Export\n"
        case 8:
            return "Start\n"
        case 9:
            return "Element\n"
        case 10:
            return "Code\n"
        case 11:
            return "Data\n"
        case 12:
            return "Data Count\n"
        case _:
            return "unknown\n"

source_file_name = 'test.wasm'
out_file_name = 'test.hex'
bytes = get_bytes(source_file_name)
# print(list(map(lambda x: "%02x" % x, bytes)));
out = open(out_file_name, 'w')
total = len(bytes)
while len(bytes) > 0:
    match bytes[0]:
        case 0:
            # Grab the Magic number
            # grab the verison number
            out.write("; Custom\n")
            for _ in range(8):
                out.write("%02x " % bytes[0])
                bytes = bytes[1:]
            out.write("\n\n")
        case 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12:
            out.write(get_section_name(bytes[0]))
            out.write("%02x " % bytes[0])
            bytes = bytes[1:]
            length = bytes[0]
            out.write("%02x " % length)
            bytes = bytes[1:]
            for i in range(length):
                out.write("%02x " % bytes[0])
                bytes = bytes[1:]
            out.write("\n\n")
        case _:
            print("Unknown byte: %02x" % bytes[0])
            print("Remaining bytes: %02x" % len(bytes))
            print(f"index: {list(map(lambda x: '%02x' % x, bytes))}")
            break

out.close()
