#[doc="graph, node, edge, and other elements"];

import core::io;
import core::option;
import std::map;
import std::map::*;

#[doc="node, edgeでの保続情報をいれるはこ"]
type attributes = map::hashmap<str, str>;

type node_id = uint;

type node_value = str;

type delaytime_pair = (edge_id, int);

#[doc="ノード"]
// TODO: valueをgenericに
type node = {
  id: node_id, 
  value: node_value, 
  mut cost: int,
  mut visited: bool, 
  visited_edge: map::hashmap<edge_id, bool>,
  mut hist: [delaytime_pair], // 遅延時間
  mut attributes: attributes
};

impl node_util for node {
  fn eql(other: node) -> bool {
    ret self.value == other.value;
  }
  fn inspect() -> str {
    ret #fmt("{id:%u,value:%s,cost:%d,hist:%s}", self.id, self.value, self.cost, vec::foldl("", self.hist){|accum, elem|
      accum + #fmt("(edge_id:%s,cost:%d),", tuple::first(elem), tuple::second(elem))
    });
  }
}

fn node(i: uint, v: str) -> @node{
  ret @{
    id : i, 
    value: v, 
    mut cost: 0,
    mut visited: false,
    visited_edge: map::str_hash::<bool>(),
    mut hist: [],
    mut attributes: map::str_hash::<str>()
  };
}

type edge_id = str;

#[doc="弧"]
type edge = {
  id: edge_id,
  source_node: @node, 
  target_node: @node, 
  weight: int, 
  mut attributes: attributes
};

impl edge_impl for edge {
  fn inspect() -> str {
    ret #fmt("{id:%s, source:%s, target:%s, weight:%d}", self.id, (*self.source_node).inspect(), (*self.target_node).inspect(), self.weight);
  }

  fn label() -> str {
    ret #fmt("(%s->%s)", self.source_node.value, self.target_node.value);
  }
}

fn edge(source: @node, target: @node, w: int) -> @edge {
  ret @{
    id: #fmt("%u_%u", source.id, target.id),
    source_node: source, 
    target_node: target, 
    weight: w, 
    mut attributes: map::str_hash::<str>()
  }
}



//TODO: 外からeqfnを貰って同値管理できるようにしないとたぶん使えない
#[doc="ノードとグラフをもつネットワークデータ的な扱い"]
type graph = {
  nodemap: map::hashmap<str, @node>,
  edgemap: map::hashmap<str, @edge>, 
  adjacency_list: map::hashmap<uint, [@edge]>
};

#[doc = "construct graph"]
fn graph() -> graph {
  ret {
    nodemap: map::str_hash::<@node>(),
    edgemap: map::str_hash::<@edge>(), 
    adjacency_list: map::uint_hash::<[@edge]>()
  };
}

impl graph_impl for graph {
  
  //TODO: いずれはgeneric
  #[doc="Append node to graph"]
  fn add_node(val: node_value) -> @node {
    if !self.nodemap.contains_key(val) {
      let n : @node = node(self.nodemap.size(), val);
      self.nodemap.insert(val, n);
    }
    ret self.nodemap.get(val);
  }
  
  #[doc="Append nodes from str to graph"]
  fn add_nodes_from(items: [node_value]) -> [@node] {
    let mut nodes : [@node] = [];
    vec::iter(items){|item|
      nodes += [self.add_node(item)];
    }

    ret nodes;
  }
  
  fn add_edge(ed: @edge) -> @edge {
    let key = ed.id;
    if !self.edgemap.contains_key(key) {
      self.edgemap.insert(key, ed);

      // update adjacency_list
      let key_adj = ed.source_node.id;
      let lst = if self.adjacency_list.contains_key(key_adj) {self.adjacency_list.get(key_adj) } else { [] } + [self.edgemap.get(key)];
      self.adjacency_list.insert(key_adj, lst);
    } else {
      fail(#fmt("Duplicate edge: %s", (*ed).inspect()));
    }
    
    ret self.edgemap.get(key);
  }
  
  fn add_edge_from_node(source:@node, target:@node, w: int) -> @edge {
    ret self.add_edge(edge(source, target, w));
  }
  
  fn node(key: node_value) -> @node {
    ret self.nodemap.get(key);
  }

  fn nodes() -> [@node] {
    let mut ns : [@node] = [];
    self.nodemap.values {|v|
      ns += [v];
    }
    ret ns;
  }

  fn edge(key: (node_value, node_value)) -> @edge {
    ret self.edgemap.get(#fmt("%u_%u", self.node(tuple::first(key)).id, self.node(tuple::second(key)).id));
  }

  fn edge_from_id(id: edge_id) -> @edge {
    ret self.edgemap.get(id);
  }

  fn edges() -> [@edge] {
    let mut eds : [@edge] = [];
    self.edgemap.values {|e|
      eds += [e];
    }
    ret eds;
  }
  
  //Return a list of successor nodes of nd
  fn successors(nd: @node) -> [@node] {
    fn get_target_node(&&ed: @edge) -> @node{
      ret ed.target_node;
    }
    ret vec::map(self.adjacency_list.get(nd.id), get_target_node);
  }
  
  //Return a list of adjacency edge of nd
  fn adjacency_edge(nd: @node) -> [@edge] {
    if self.adjacency_list.contains_key(nd.id) {
      ret self.adjacency_list.get(nd.id);
    } else {
      ret [];
    }
  }
}


#[doc="dump to graph ml"]
fn dump(g: graph) -> str {
  let mut xml = "<?xml version='1.0' encoding='UTF-8'?>\n\
<graphml xmlns='http://graphml.graphdrawing.org/xmlns'\n \
    xmlns:xsi='http://www.w3.org/2001/XMLSchema-instance'\n \
    xsi:schemaLocation='http://graphml.graphdrawing.org/xmlns\n \
     http://graphml.graphdrawing.org/xmlns/1.0/graphml.xsd'>\n";
  xml += "<graph id='G' edgedefault='directed'>\n";
  
  xml += #fmt("  <key attr.name='edge.label' attr.type='string' for='node' id='edge.label' />\
    <key attr.name='node.label' attr.type='string' for='node' id='node.label' />
    <key attr.name='node.showNestedNetwork' attr.type='string' for='node' id='node.showNestedNetwork'/>
    <key attr.name='node.labelColor' attr.type='string' for='node' id='node.labelColor'/>
    <key attr.name='node.fillColor' attr.type='string' for='node' id='node.fillColor'/>
    <key attr.name='hiddenLabel' attr.type='string' for='node' id='hiddenLabel'/>
    <key attr.name='node.size' attr.type='string' for='node' id='node.size'/>
    <key attr.name='nested_network_id' attr.type='string' for='node' id='nested_network_id'/>
    <key attr.name='NODE_TYPE' attr.type='string' for='node' id='NODE_TYPE'/>
    <key attr.name='node.lineStyle' attr.type='string' for='node' id='node.lineStyle'/>
    <key attr.name='node.borderColor' attr.type='string' for='node' id='node.borderColor'/>
    <key attr.name='node.shape' attr.type='string' for='node' id='node.shape'/>
    <key attr.name='canonicalName' attr.type='string' for='node' id='canonicalName'/>
    <key attr.name='has_nested_network' attr.type='string' for='node' id='has_nested_network'/>
    <key attr.name='nested_network_is_visible' attr.type='boolean' for='node' id='nested_network_is_visible'/>
    <key attr.name='node.label' attr.type='string' for='node' id='node.label'/>
    <key attr.name='node.toolTip' attr.type='string' for='node' id='node.toolTip'/>
    <key attr.name='node.customGraphics1' attr.type='string' for='node' id='node.customGraphics1'/>
    <key attr.name='edge.lineStyle' attr.type='string' for='node' id='edge.lineStyle'/>
    <key attr.name='canonicalName' attr.type='string' for='node' id='canonicalName'/>
    <key attr.name='interaction' attr.type='string' for='node' id='interaction'/>
    <key attr.name='parent_nodes' attr.type='string' for='node' id='parent_nodes'/>
    <key attr.name='node.labelPosition' attr.type='string' for='node' id='node.labelPosition'/>
    <key attr.name='__layoutAlgorithm' attr.type='string' for='node' id='__layoutAlgorithm'/>\n\n");
  

  for g.nodes().each {|n|
    let node_id = n.value;
    let mut node_label = n.value;
    
    let labels = vec::map(n.hist) {|his|
      let (edg_id, cost) = his;
      let edg = g.edge_from_id(edg_id);
      let source_label = edg.source_node.value;
      #fmt("(%s, %d)", source_label, cost)
    };
    
    xml += #fmt("   <node id='%s'><data key='node.label'>%s</data></node>\n", node_id,  node_label + vec::foldl("", labels) {|ac, a|
      ac + a
    });
    
   //  vec::eachi(n.hist){|ix, hist|
   //    let edg = g.edge_from_id(tuple::first(hist));
   //    let source_label = edg.source_node.value;
      
   //    let inner_label = #fmt("(%s, %d)", source_label, tuple::second(hist));
   //    let delaytime_node_id = #fmt("delaytime_%s_%u", node_label, ix);
   //    xml += #fmt("   <node id='%s'>
   //      <data key='node.label'>%s</data>
   //      <data key='node.labelColor'>51,51,51</data>
   //      <data key='node.fillColor'>153,153,153</data>
   //      <data key='node.size'>1.0</data>
   //      <data key='NODE_TYPE'>DefaultNode</data>
   //      <data key='node.lineStyle'>PARALLEL_LINES</data>
   //      <data key='node.borderColor'>0,0,0</data>
   //      <data key='node.shape'>rect</data>
   //      <data key='node.labelPosition'>SW,NW,c,0.00,0.00</data>
   // </node>\n", delaytime_node_id, inner_label);
      
   //    xml += #fmt("   <edge id='edge_delay_%s_%u' source='%s' target='%s'><data key='edge.lineStyle'>SINEWAVE</data></edge>", 
   //                node_label, ix, delaytime_node_id, node_label);
   //    true
   //  }

    
  }
  
  vec::iter(g.edges()) {|ed|
    xml += #fmt("   <edge id='%s' directed='true' source='%s' target='%s'><data key='edge.label'>%d</data></edge>\n", 
                ed.id,
                ed.source_node.value, 
                ed.target_node.value, 
                ed.weight
                );
  }

  // 遅延時間の表示
  
  
  xml += "</graph>\n";
  xml += "</graphml>";
  log(error, xml);
  ret xml;
}

#[doc="dump to file"]
fn dump_file(g: graph, path: str) {
  alt io::mk_file_writer(path, [io::create]) {
   result::err(e) {
    log(error, e);
   }
   result::ok(writer) {
    writer.write(str::bytes(dump(g)));
   }
  }
}

#[cfg(test)]
mod test{
  #[test]
  fn test_init(){
    let g = graph();
    assert(g.nodemap.size()==0u);
    assert(g.edgemap.size()==0u);
  }
  
  #[test]
  fn test_edge(){
    let g = graph();
    let s = g.add_node("発");
    let t = g.add_node("着");
    
    assert(s.id == 0u);
    assert(t.id == 1u);
    assert(s.value == "発");
    assert(t.value == "着");

    let edge = edge(s, t, 0);
    assert(edge.id == "0_1");

    assert(g.node("発") == s);
  }
  
  #[test]
  fn test_node() {
    let g = graph();
    let n1 = g.add_node("ああああ");
    assert(n1.id==0u);
    assert(n1.value=="ああああ");

    // ポインタ参照であるのでこの変更が反映されているかちぇっく
    n1.attributes.insert("hoge", "fooo");
    assert(g.nodemap.get(n1.value).attributes.get("hoge") == "fooo");
  }

  #[test]
  fn test_nodes() {
    let g = graph();
    let nv = g.add_nodes_from(["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "O", "M", "Z"]);
    assert(g.nodemap.size() == 15u);
    assert(g.node("A") == nv[0]);
  }

  
}


/*
see:
http://networkx.lanl.gov/tutorial/tutorial.html
http://doc.rust-lang.org/doc/tutorial.html

*/



