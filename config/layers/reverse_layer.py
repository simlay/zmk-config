#!/usr/bin/env python3
"""Reverse a ZMK layer's bindings (mirror horizontally).

Usage:
    python3 reverse_layer.py source.dtsi target.dtsi

Example:
    python3 reverse_layer.py base.dtsi reversebase.dtsi
    python3 reverse_layer.py symbols.dtsi reverssymbols.dtsi
"""
import sys
import re

def extract_bindings(text):
    """Extract individual bindings like &kp TAB, &trans, &mo 1, etc."""
    # Match &word or &word word patterns
    return re.findall(r'&\w+(?:\s+\w+)?', text)

def reverse_layer(input_file, output_file):
    with open(input_file, 'r') as f:
        content = f.read()
    
    # Extract layer name from output filename
    import os
    layer_name = os.path.splitext(os.path.basename(output_file))[0]
    
    # Extract bindings content (everything between < and >)
    bindings_match = re.search(r'bindings\s*=\s*(<.*?>)', content, re.DOTALL)
    if not bindings_match:
        print(f"Error: Could not find bindings in {input_file}", file=sys.stderr)
        sys.exit(1)
    
    bindings_content = bindings_match.group(1)
    
    # Extract all bindings
    all_bindings = extract_bindings(bindings_content)
    
    # We have 4 rows: 3 rows of 12, 1 row of 6 (with leading spaces)
    # Split into rows based on the original structure
    lines = bindings_content[1:-1].strip().split('\n')
    
    # Process each line: extract bindings, reverse them
    reversed_lines = []
    for line in lines:
        line_bindings = extract_bindings(line)
        if line_bindings:
            # Reverse the bindings in this row
            line_bindings = line_bindings[::-1]
            reversed_lines.append('    ' + '    '.join(line_bindings))
        else:
            reversed_lines.append('')
    
    # Join reversed lines
    reversed_bindings = '<\n' + '\n'.join(reversed_lines) + '\n    >'
    
    # Write output
    output = f'''{layer_name} {{
    bindings = {reversed_bindings};
}}
'''
    
    with open(output_file, 'w') as f:
        f.write(output)
    
    print(f"Reversed {input_file} -> {output_file}")

if __name__ == '__main__':
    if len(sys.argv) != 3:
        print("Usage: python3 reverse_layer.py source.dtsi target.dtsi", file=sys.stderr)
        sys.exit(1)
    
    reverse_layer(sys.argv[1], sys.argv[2])
