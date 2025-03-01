import xml.etree.ElementTree as ET
import re, os, textwrap


def wrap_para(para: str) -> str:
    return '\n  '.join(textwrap.wrap(para))


def expect(x, msg: str):
    if x is None:
        raise RuntimeError(msg)
    else:
        return x


def fmt_constant(name: str) -> str:
    if name.startswith("GL_"):
        name = "gl::" + name[len("GL_"):]
    return f"[`{name}`]"


def fmt_function(name: str) -> str:
    if name.startswith("gl_"):
        name = name[len("gl_"):]
    if name.startswith("gl"):
        name = name[len("gl"):]
    snake_case = re.sub(r'(?<=[a-z])(?=[A-Z])|(?<=\w)(?=[A-Z][a-z])', '_', name).lower()
    return f"[`Gl::{snake_case}`]"


def clean_para(para: ET.Element, ns) -> str:
    line: str = ""
    for item in para.iter():
        if item.tag:
            tag = item.tag.split('}')
            tag = tag[len(tag) - 1]
            if tag in ["para", "mrow", "mn"]:
                item_text = item.text
            elif tag == "link":
                item_text = f"[{item.text}]({item.text})"
            elif tag in ["msub"]:
                item_text = f"_{{{item.text}}}"
            elif tag in ["msup"]:
                item_text = f"^{{{item.text}}}"
            elif tag == "constant":
                item_text = fmt_constant(item.text)
            elif tag in ["inlineequation", "informalequation"]:
                item_text = f"${item.text}$"
            elif tag == "math":
                item_text = f"\n$$\n{item.text}\n$$\n"
            elif tag in ["function", "varname"]:
                item_text = fmt_function(item.text)
            elif tag in ["parameter", "c"]:
                item_text = f"`{item.text}`"
            elif tag in ["code", "programlisting", "mfenced"]:
                item_text = f"\n```c\n{item.text}\n```\n"
            elif tag in ["emphasis", "mi", "mo"]:
                item_text = f"*{item.text}*"
            elif tag == "footnote":
                item_text = f"({item.text})"
            elif tag in ["citerefentry"]:
                item_text = fmt_function(expect(item.find("db:refentrytitle", ns), "citerefentry should have refentrytitle").text)
            elif tag == "refentrytitle":
                item_text = ""
            else:
                raise RuntimeError(f"unexpected tag: {tag}")
            line += item_text
        elif item.text:
            item_text = item.text
            line += item_text
        if item.tail:
            item_text = item.tail
            line += item_text
    line = re.sub(r'\s+', ' ', line.strip())
    return wrap_para(line)


if __name__ == "__main__":
    for filename in os.listdir("./"):
        if filename.endswith(".xml"):
            with open(filename, "r", encoding="utf8") as file:
                print(f"{filename:<56}", end=" ")
                try:
                    file_text = file.read()
                    for (a, b) in [
                        ("&it;", "\\u{2062}"),
                        ("&plus;", "\\u{002B}"),
                        ("&minus;", "\\u{2212}"),
                        ("&nbsp;", " "),
                        ("&le;", "\\u{2264}"),
                        ("&ge;", "\\u{2265}"),
                        ("&ne;", "\\u{2260}"),
                        ("&af;", "\\u{2061}"),
                        ("&times;", "\\u{00D7}"),
                        ("&infin;", "\\u{221E}"),
                        ("&delta;", "\\u{03B4}"),
                        ("&Delta;", "\\u{0394}"),
                        ("&lambda;", "\\u{03BB}"),
                        ("&sdot;", "\\u{22C5}"),
                    ]:
                        file_text = file_text.replace(a, b)
                    root = ET.fromstring(file_text)
                except Exception as e:
                    print(f"[ERROR: parse failed: {e}]")
                    continue

                try:
                    content = ""
                    refnamediv = root.find('refnamediv')
                    if refnamediv is None:
                        ns = {
                            'db': "http://docbook.org/ns/docbook",
                            'mml': "http://www.oasis-open.org/docbook/xml/mathml/1.1CR1/dbmathml.dtd",
                        }
                        refnamediv = root.find('db:refnamediv', ns)
                    else:
                        ns = {
                            'db': "",
                            'mml': "",
                        }
                    name = expect(refnamediv.find('db:refname', ns), "missing name").text
                    content += f"# {name}\n"
                    brief = expect(refnamediv.find('db:refpurpose', ns), "missing brief").text
                    content += wrap_para(brief) + '\n'
                    refsects = root.findall('db:refsect1', ns)
                    for sect in refsects:
                        title = expect(sect.find('db:title', ns), "missing title").text
                        if title not in ["Associated Gets", "Version Support", "Copyright"]:
                            content += f"\n## {title}\n"
                            if title == "Parameters":
                                for variable in sect.findall('db:variablelist', ns):
                                    entry = variable.find('db:varlistentry', ns)
                                    param = entry.find('db:term/db:parameter', ns).text
                                    content += f"- `{param}`\n"
                                    param_def = '\n'.join([f"  {clean_para(para, ns)}\n" for para in entry.findall('db:listitem/db:para', ns)])
                                    content += param_def
                            elif title in ["Description", "Notes", "Overview", "Example", "Examples", "Using GLX Extensions", "GLX 1.1, GLX 1.2, and GLX 1.3"]:
                                pars = sect.findall('db:para', ns)
                                for par in pars:
                                    content += clean_para(par, ns) + '\n'
                            elif title == "Errors":
                                pars = sect.findall('db:para', ns)
                                err_list = ''.join([f"- {clean_para(par, ns)}\n" for par in pars])
                                content += err_list
                            elif title == "See Also":
                                names = sect.findall('db:para/db:citerefentry/db:refentrytitle', ns)
                                see_also_list = ''.join([f"- {fmt_function(name.text)}\n" for name in names])
                                content += see_also_list
                            else:
                                raise RuntimeError(f"unexpected section: {title}")

                    # print(content)
                    with open(filename.replace(".xml", ".md"), "w", encoding="utf-8") as md_file:
                        md_file.write(content)
                    print()
                except Exception as e:
                    print(f"[ERROR: {e}]")
                    # def debug_layer(node, indent: str):
                    #     print(indent, end="")
                    #     print(node)
                    #     for item in node:
                    #         debug_layer(item, indent + "  ")
                    # print("structure:")
                    # debug_layer(root, "")
