import sys

def get_bytes(filename):
    with open(filename, 'rb') as f:
        return f.read()

def get_section_name(id: int) -> str:
    match id:
        case 0:
            return "Custom"
        case 1:
            return "Type"
        case 2:
            return "Import"
        case 3:
            return "Function"
        case 4:
            return "Table"
        case 5:
            return "Memory"
        case 6:
            return "Global"
        case 7:
            return "Export"
        case 8:
            return "Start"
        case 9:
            return "Element"
        case 10:
            return "Code"
        case 11:
            return "Data"
        case 12:
            return "Data Count"
        case _:
            return "unknown"

# # print(list(map(lambda x: "%02x" % x, bytes)));
# out = open(out_file_name, 'w')
# total = len(bytes)
#
# # Grab the Magic number
# # grab the verison number
# out.write("; Header\n")
# for _ in range(8):
#     out.write("%02x " % bytes[0])
#     bytes = bytes[1:]
# out.write("\n\n")
# while len(bytes) > 0:
#     if bytes[0] == 0x01:
#         out.write(f"---- {get_section_name(bytes[0])} ----\n")
#         out.write("%02x ID\n" % bytes[0])
#         bytes = bytes[1:]
#         length = bytes[0]
#         out.write("%02x Length\n" % length)
#         bytes = bytes[1:]
#         out.write("%02x Count\n" % bytes[0])
#         bytes = bytes[1:]
#         for i in range(1, length):
#             out.write("%02x " % bytes[0])
#             bytes = bytes[1:]
#         out.write("\n\n")
#
#     elif bytes[0] in range(1, 13):
#         out.write(f"---- {get_section_name(bytes[0])} ----\n")
#         out.write("%02x " % bytes[0])
#         bytes = bytes[1:]
#         length = bytes[0]
#         out.write("%02x " % length)
#         bytes = bytes[1:]
#         for i in range(length):
#             out.write("%02x " % bytes[0])
#             bytes = bytes[1:]
#         out.write("\n\n")
#
#     else:
#         print("Unknown byte: %02x" % bytes[0])
#         print("Remaining bytes: %02x" % len(bytes))
#         print(f"index: {list(map(lambda x: '%02x' % x, bytes))}")
#         break
#
# out.close()

class Decoder:
    def __init__(self, bytes):
        self.bytes = bytes
        self.index = 0
        self.current = None

    def next_byte(self):
        if self.index >= len(self.bytes):
            return None
        self.current = self.bytes[self.index]
        self.index += 1
        return self.current

    def decode_header(self) -> str:
        out = ""
        out += "; Header\n"
        for _ in range(8):
            out += f"{self.next_byte():02x} "
        out += "\n\n"
        return out

    def decode_type_section(self):
        if self.current is None:
            print("No current byte")
            sys.exit(1)

        out = ""
        out += f"---- {get_section_name(self.current)} ----\n"
        out += f"{self.current:02x} ID\n"

        length = self.next_byte()
        if length is None:
            print("No length")
            sys.exit(1)

        out += f"{length:02x} Length\n"
        out += f"{self.next_byte():02x} Count\n"
        for _ in range(1, length):
            out += f"{self.next_byte():02x} "
        out += "\n\n"

        return out


    def decode(self) -> str:
        out = self.decode_header()
        while True:
            byte = self.next_byte()
            if byte is None:
                break
            yield byte
            if byte == 0x01:
                out += self.decode_type_section()
            elif byte in range(1, 13):
                out += f"---- {get_section_name(byte)} ----\n"
                out += f"{byte:02x} "
                length = self.next_byte()
                if length is None:
                    break
                out += f"{length:02x} "
                out += f"{length:%02x} "
                for _ in range(length):
                    byte = self.next_byte()
                    if byte is None:
                        break
                    out += f"{byte:02x} "
                out += "\n\n"

            else:
                print("Unknown byte: %02x" % byte)
                print("Remaining bytes: %02x" % len(self.bytes))
                print(f"index: {list(map(lambda x: '%02x' % x, self.bytes))}")
                break

        return out


if __name__ == "__main__":
    args = sys.argv

    if len(args) < 2:
        print("Usage: inspect_wasm_binary.py <filename>")
        print("example: inspect_wasm_binary.py hello.wasm")
        sys.exit(1)

    if args[1] in ["-h", "--help"]:
        print("Usage: inspect_wasm_binary.py <filename> [output_file]")
        sys.exit(0)

    source_file_name = args[1]
    out_file_name = args[2] if len(args) > 2 else source_file_name.split(".")[0] + ".hex"
    bytes = get_bytes(source_file_name)
    decoder = Decoder(bytes)
    with open(out_file_name, 'w') as out:
        decoded_bytes = decoder.decode()
        out.write(decoded_bytes)
