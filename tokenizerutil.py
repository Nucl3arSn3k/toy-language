import os
import re

#just a debugging util
def main():
    ast_split()
    print("splitting tokenizer input")
    input_file = open("tokendump.txt",'r')
    output_file = open("tokendumpv2.txt",'w')
    content = input_file.read()
    input_file.close()
    individ = content.split(',')
    for x in individ:
        output_file.write(x.strip() + '\n   ')
    os.remove("tokendump.txt")
    

def ast_split():
    input_file = open("ASTdump.txt", 'r')
    output_file = open("ASTdumpv2.txt", 'w')
    content = input_file.read()
    input_file.close()
    
    # Remove outer brackets and split on }, but keep the }
    content = content.strip('[]')
    nodes = content.split('}, ')
    
    # Process each node
    for i, node in enumerate(nodes):
        # Add back the } if it's not the last element
        if i < len(nodes) - 1:
            node = node + '}'
            
        # Remove the node type (VariableDeclaration, ExpressionStatement, etc)
        if ' { ' in node:
            _, inner_content = node.split(' { ', 1)
            output_file.write(inner_content + '\n   ')
        else:
            # For the last node (DisplayIntStatement) just write it directly
            output_file.write(node + '\n   ')
    
    output_file.close()
    os.remove("ASTdump.txt")

if __name__ == "__main__":
    main()