import os



def main():
    print("splitting tokenizer input")
    input_file = open("tokendump.txt",'r')
    output_file = open("tokendumpv2.txt",'w')
    content = input_file.read()
    individ = content.split(',')
    for x in individ:
        output_file.write(x.strip() + '\n   ')
    os.remove("tokendump.txt")

    
    






if __name__ == "__main__":
    main()