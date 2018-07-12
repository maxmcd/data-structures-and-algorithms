"""

Node {
    String value;
    Set<Node> neighbors;
}

A<->B<->C
    \  /
     D
     

A: [B]
B: [A, C, D]
C: [B, D]
D: [B, C]


Node createDeepCopy(Node node);

"""


class Node:

    def __init__(self, value):
        self.value = value
        self.neighbors = set()


def createDeepCopy(node):
    visited = {}
    copy = Node(node.value)
    visit(visited, node, copy)
    return copy


def visit(visited, node, copy):
    for neighbor in node.neighbors:
        if not visited.get(neighbor.value):
            new_node = Node(neighbor.value)
            copy.neighbors.add(new_node)
            visited[neighbor.value] = new_node
            visit(visited, neighbor, new_node)
        else:
            visited[neighbor.value].neighbors.add(copy)


a = Node("A")
b = Node("B")
c = Node("C")
d = Node("D")
a.neighbors = [b]
b.neighbors = [a, c, d]
c.neighbors = [b, d]
d.neighbors = [b, c]

result = createDeepCopy(a)
print(next(iter(result.neighbors))).value
print([x.value for x in next(iter(result.neighbors)).neighbors])
