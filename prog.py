from rdflib import Graph, term
from pandas import DataFrame
import numpy as np

def main():
    graph = Graph()
    graph.parse("beatles.rdf", format="turtle")

    adj_matrix = get_adj_matrix(graph)
    print(adj_matrix)

def get_adj_matrix(g: Graph) -> DataFrame:
    vertices = []
    sub_obj_pairs = []

    for sub, obj in g.subject_objects():
        if type(obj) == term.Literal:
            continue
        sub_obj_pairs.append([sub.toPython(), obj.toPython()])
        vertices.append(sub.toPython())
        vertices.append(obj.toPython())
    
    vertices = list(set(vertices))
    df = DataFrame(columns=vertices, index=vertices)
    df.fillna(0, inplace=True)

    for sub, obj in sub_obj_pairs:
        if type(obj) == term.Literal:
            continue
        df[obj][sub] = 1

    return df

def remove_duplicates(arr):
    return list(dict.fromkeys(arr))

def remove_prefix(s):
    if s.startswith("http:"):
        return s[s.rindex("/") + 1:]
    else:
        return s

if __name__ == "__main__":
    main()