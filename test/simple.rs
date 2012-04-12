
use std;
use network;

import network::engine;
import network::graph;
import network::graph::*;

import network::heap;

fn build_network() -> graph {
  let g = graph();
  let nv = g.add_nodes_from(["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "O", "M", "Z"]);
  // let edg = edge(g.node("A"), g.node("B"), 4);
  nv;
  
  g.add_edge_from_node(g.node("A"), g.node("B"), 4);
  g.add_edge_from_node(g.node("E"), g.node("B"), 5);
  g.add_edge_from_node(g.node("B"), g.node("C"), 10);
  g.add_edge_from_node(g.node("C"), g.node("F"), 50);
  g.add_edge_from_node(g.node("C"), g.node("D"), 10);
  g.add_edge_from_node(g.node("D"), g.node("G"), 50);
  g.add_edge_from_node(g.node("C"), g.node("G"), 20);
  g.add_edge_from_node(g.node("G"), g.node("H"), 1);
  g.add_edge_from_node(g.node("H"), g.node("F"), 2);
  g.add_edge_from_node(g.node("H"), g.node("I"), 9);
  g.add_edge_from_node(g.node("H"), g.node("K"), 8);
  g.add_edge_from_node(g.node("H"), g.node("J"), 5);
  g.add_edge_from_node(g.node("J"), g.node("L"), 2);
  g.add_edge_from_node(g.node("K"), g.node("L"), 4);
  g.add_edge_from_node(g.node("I"), g.node("O"), 1);
  g.add_edge_from_node(g.node("O"), g.node("M"), 1);
  g.add_edge_from_node(g.node("M"), g.node("Z"), 15);
  g.add_edge_from_node(g.node("L"), g.node("Z"), 16);
  
  ret g;
}


fn build_network2() -> graph {
  let g = graph();
  let nv = g.add_nodes_from(["A", "B", "C", "D", "E", "F", "G"]);
  // let edg = edge(g.node("A"), g.node("B"), 4);
  nv;
  
  g.add_edge_from_node(g.node("A"), g.node("G"), 10);
  g.add_edge_from_node(g.node("A"), g.node("B"), 10);
  g.add_edge_from_node(g.node("G"), g.node("B"), 50);
  g.add_edge_from_node(g.node("B"), g.node("C"), 20);
  g.add_edge_from_node(g.node("A"), g.node("F"), 60);
  g.add_edge_from_node(g.node("F"), g.node("E"), 10);
  g.add_edge_from_node(g.node("E"), g.node("C"), 5);
  g.add_edge_from_node(g.node("C"), g.node("D"), 30);
  g.add_edge_from_node(g.node("E"), g.node("D"), 10);
  
  ret g;
}


fn build_network3() -> graph {
  let g = graph();
  g.add_nodes_from(["A", "B", "C", "D", "E", "F", "G", "H"]);
  
  g.add_edge_from_node(g.node("A"), g.node("G"), 10);
  g.add_edge_from_node(g.node("A"), g.node("B"), 10);
  g.add_edge_from_node(g.node("G"), g.node("B"), 50);
  g.add_edge_from_node(g.node("B"), g.node("C"), 20);
  g.add_edge_from_node(g.node("A"), g.node("F"), 60);
  g.add_edge_from_node(g.node("F"), g.node("E"), 10);
  g.add_edge_from_node(g.node("E"), g.node("C"), 5);
  g.add_edge_from_node(g.node("C"), g.node("D"), 30);
  g.add_edge_from_node(g.node("E"), g.node("D"), 10);
  g.add_edge_from_node(g.node("A"), g.node("H"), 70);
  g.add_edge_from_node(g.node("H"), g.node("E"), 10);
  ret g;
}

fn build_network4() -> graph {
  let g = graph();
  g.add_nodes_from(["A", "B", "C", "D", "E", "F", "G", "H", "Z"]);
  
  g.add_edge_from_node(g.node("A"), g.node("B"), 1);
  g.add_edge_from_node(g.node("B"), g.node("C"), 1);
  g.add_edge_from_node(g.node("C"), g.node("D"), 10);
  g.add_edge_from_node(g.node("C"), g.node("E"), 20);
  g.add_edge_from_node(g.node("C"), g.node("F"), 30);

  g.add_edge_from_node(g.node("D"), g.node("E"), 13);

  g.add_edge_from_node(g.node("D"), g.node("G"), 70);
  g.add_edge_from_node(g.node("E"), g.node("H"), 60);
  g.add_edge_from_node(g.node("F"), g.node("Z"), 68);
  
  g.add_edge_from_node(g.node("G"), g.node("H"), 5);
  g.add_edge_from_node(g.node("H"), g.node("Z"), 10);

  ret g;
}

fn build_network5() -> graph {
  let g = graph();
  g.add_nodes_from(["A", "B", "C", "D", "E", "F", "G", "H"]);
  
  g.add_edge_from_node(g.node("A"), g.node("B"), 20);
  g.add_edge_from_node(g.node("A"), g.node("G"), 10);
  g.add_edge_from_node(g.node("G"), g.node("B"), 5);
  g.add_edge_from_node(g.node("A"), g.node("H"), 1);

  g.add_edge_from_node(g.node("B"), g.node("C"), 6);
  g.add_edge_from_node(g.node("H"), g.node("C"), 5);
  g.add_edge_from_node(g.node("H"), g.node("F"), 2);
  g.add_edge_from_node(g.node("H"), g.node("E"), 50);

  g.add_edge_from_node(g.node("C"), g.node("D"), 4);
  g.add_edge_from_node(g.node("F"), g.node("D"), 3);

  g.add_edge_from_node(g.node("D"), g.node("E"), 8);
  ret g;
}

#[cfg(test)]
mod test { 
  #[test]
  fn sync(){
    let g = build_network();
    fn leual(&&a: @node,  &&b: @node) -> bool {
      ret a.cost > b.cost;
    };
    
    let f = leual;
    let pq = heap::create::<@node>(f);
    
    g.node("A").cost = 9;
    // log(error, (*g.node("A")).inspect());
    assert(g.node("A").cost == 9);
    
    heap::push(pq, g.node("A"));
    // log(error, (*option::get(heap::top(pq))).inspect());
    assert(option::get(heap::top(pq)).cost == 9);
    
    let n = option::get(heap::pop(pq));
    n;
  }
  
  #[test]
  fn heap_tuple() {
    type delay_t = (int, int);
    fn le(&&a: delay_t, &&b: delay_t) -> bool {
      ret tuple::first(a) > tuple::first(b);
    }
    let f = le;
    let pq = heap::create::<delay_t>(f);
    
    heap::push(pq, (1, 3));
    heap::push(pq, (10, 1));
  }
  
  #[test]
  fn simple(){
    let g = build_network();
    let successors_b = g.successors(g.node("B"));
    assert(successors_b == [g.node("C")]);
    let successors_c = g.successors(g.node("C"));
    assert(vec::len(successors_c) == 3u);
    assert(successors_c == [g.node("F"), g.node("D"), g.node("G")]);
    
    let e1 = g.edge(("A", "B"));
    // log(error, (*e1).inspect());
    e1.target_node.cost = 10;
    
    // log(error, (*e1).inspect());
    
    let e2 = g.edge(("A", "B"));
    // log(error, (*e2).inspect());
    
    e2.source_node.cost = 99;
    assert(e1.source_node.cost == 99);
    
    for g.adjacency_edge(e2.source_node).each {|nx|
      assert(nx.source_node.cost == e2.source_node.cost);
    }
  }
  
  #[test]
  fn test_adjacency_edge(){
    let g = build_network();
    assert(g.adjacency_edge(g.node("Z")) == []);
  }
  
  #[test]
  fn dijkstra_path() {
    let g = build_network();
    let result = engine::dijkstra_path(g, [g.node("B"), g.node("F")]);
    alt result {
     core::result::ok(path) {
      assert(path == [g.node("B"), g.node("C"), g.node("G"), g.node("H"), g.node("F")]);
     }
     result::err(m) {
      log(error, m);
      fail;
     }
    }
  }
  

  #[test]
  fn dijkstra_n_path() {
    let g = build_network();
    let result = engine::dijkstra_n_path(g, [g.node("A"), g.node("Z")], 5u);
    
    alt result {
     core::result::ok(pathes) {
      assert(vec::len(pathes) <= 5u);
      fn concat(&&ac: str, &&a: @node) -> str {
        ret ac+#fmt("%s->", a.value);
      }
      
      vec::iter(pathes) {|path|
        log(error, vec::foldl("", path, concat));
      }
      
     }
     result::err(m) {
      log(error, m);
      fail;
     }
    }
  }



  #[test]
  fn dijkstra_search2() {
    let g = build_network3();
    engine::dijkstra_search(g, [g.node("A"), g.node("D")], engine::widespread);
    assert(vec::len(g.node("C").hist) == 2u);
  }

  
  #[test]
  fn dijkstra_n_path3(){
    let g = build_network4();
    // graph::dump_file( g, "test.graphml");
    
    let result = engine::dijkstra_n_path(g, [g.node("A"), g.node("Z")], 5u);
    
    alt result {
     core::result::ok(pathes) {
      fn concat(&&ac: str, &&a: @node) -> str {
        ret ac+#fmt("%s->", a.value);
      }
      vec::iter(pathes) {|path|
        log(error, vec::foldl("", path, concat));
      }
      
      assert(vec::len(pathes) != 0u);
      assert(vec::len(pathes) <= 5u);
      
      fn nodes_from_vec(g: graph::graph, v: [str]) -> [@node] {
        ret vec::map(v) {|node_label|
          g.node(node_label)
        }
      }
     }
     result::err(m) {
      log(error, m);
      fail;
     }
    }
  }
  
  #[test]
  fn dijkstra_n_path2(){
    let g = build_network3();
    let result = engine::dijkstra_n_path(g, [g.node("A"), g.node("D")], 10u);
    
    alt result {
     core::result::ok(pathes) {
      fn concat(&&ac: str, &&a: @node) -> str {
        ret ac+#fmt("%s->", a.value);
      }
      vec::iter(pathes) {|path|
        log(error, vec::foldl("", path, concat));
      }
      
      assert(vec::len(pathes) != 0u);
      assert(vec::len(pathes) <= 5u);
      
      fn nodes_from_vec(g: graph::graph, v: [str]) -> [@node] {
        ret vec::map(v) {|node_label|
          g.node(node_label)
        }
      }
      
      assert(pathes == [
        nodes_from_vec(g, ["A", "B", "C", "D"]), 
        nodes_from_vec(g, ["A", "F", "E", "D"]), 
        nodes_from_vec(g, ["A", "H", "E", "D"]), 
        nodes_from_vec(g, ["A", "F", "E", "C", "D"]), 
        nodes_from_vec(g, ["A", "G", "B", "C", "D"])]);
      
     }
     result::err(m) {
      log(error, m);
      fail;
     }
    }
  }
  
  #[test]
  fn dijkstra_n_path5(){
    let g = build_network5();
    let result = engine::dijkstra_n_path(g, [g.node("A"), g.node("E")], 5u);
    alt result {
     core::result::ok(pathes) {
      fn concat(&&ac: str, &&a: @node) -> str {
        ret ac+#fmt("%s->", a.value);
      }
      vec::iter(pathes) {|path|
        log(error, vec::foldl("", path, concat));
      }
      
      assert(vec::len(pathes) == 5u);
      fn nodes_from_vec(g: graph::graph, v: [str]) -> [@node] {
        ret vec::map(v) {|node_label|
          g.node(node_label)
        }
      }
      
      assert(pathes == [
        nodes_from_vec(g, ["A", "H" ,"F", "D", "E"]), 
        nodes_from_vec(g, ["A", "H", "C", "D", "E"]), 
        nodes_from_vec(g, ["A", "G", "B", "C", "D", "E"]), 
        nodes_from_vec(g, ["A", "B", "C", "D", "E"]), 
        nodes_from_vec(g, ["A", "H", "E"])]);
     }
     result::err(m) {
      log(error, m);
      fail;
     }
    }
  }



}
