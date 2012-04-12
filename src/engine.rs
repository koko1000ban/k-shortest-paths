
import graph;
import graph::*;

import core::result;
import core::result::*;

import std::map;
import std::map::*;

import std::sort;

import heap;

/*
ノードごとに到達コストを持ち、それが多数管理されていれば第2経路なども導出できる
結果生成は、ゴールノードから逆にたどる
*/
enum search_mode  {
  shortest,
  widespread
}

impl extensions<T:copy> for [const T] {
  #[doc = "Return first element"]
  #[inline]
  fn head() -> T { vec::head(self) }
}

impl tuple_util<T:copy, U:copy> for (T, U) {
  fn first() -> T {
    ret tuple::first(self);
  }
  
  fn second() -> U {
    ret tuple::second(self);
  }
}

#[doc="search with dijkstra"]
fn dijkstra_search(g: graph::graph, via: [@node], mode: search_mode) -> result<str, str> {
  type node_candidate = (int, (@node, edge_id));
  
  fn leual(&&a: node_candidate,  &&b: node_candidate) -> bool {
    ret tuple::first(a) > tuple::first(b);
  };
  let f = leual;
  let pq = heap::create::<node_candidate>(f);
  
  // TODO: 途中駅があった場合に、並列処理する
  // 並列処理した場合、遅延木はどうすんだ? visitorパターンも
  let start_node = via[0u];
  let goal_node = via[vec::len(via)-1u];
  #debug("start:%s goal:%s", (*start_node).inspect(), (*goal_node).inspect());

  // push adjacency edge of start node
  vec::iter(g.adjacency_edge(start_node)) {|edge|
    // edge.target_node.cost += edge.weight;    
    let next_cost = edge.source_node.cost + edge.weight;
    heap::push(pq, (next_cost, (edge.target_node, edge.id)));
    #debug("push (%d, %s)", next_cost , (*edge.target_node).inspect());
  }
  
  assert(pq.nelts != 0u);  
  assert(heap::empty(pq) == false);
  
  while !heap::empty(pq) {

    log(debug, "- - - - - - ");
    let (cost, (current_node, pre_edge)) = option::get(heap::pop(pq));
    
    if current_node.visited {
      if !current_node.visited_edge.contains_key(pre_edge) {
        // update delaytime
        log(debug, "update delay times");
        current_node.visited_edge.insert(pre_edge, true);
        current_node.hist += [(pre_edge, cost - current_node.cost)];
      }
      
    } else {

      // visit node
      current_node.cost = cost;
      current_node.visited = true;
      current_node.visited_edge.insert(pre_edge, true);
      current_node.hist = [(pre_edge, 0)];
      
      #debug("current node is %s", (*current_node).inspect());
      if (*goal_node).eql(*current_node) {
        log(debug, "reach goal node");
        
        if mode == shortest {
          break;
        }

      }
      
      for g.adjacency_edge(current_node).each {|next_edge|

        #debug("adj_edge: %s", (*next_edge).inspect());
        assert(next_edge.source_node.cost == current_node.cost);
        let next_cost = next_edge.source_node.cost + next_edge.weight;

        
        if next_edge.target_node.visited {
          // already visited
          log(debug, "already visited"); 
          assert(vec::len(next_edge.target_node.hist) != 0u);
          assert(next_edge.target_node.cost <= next_cost);
          
          // まったく同じ内容の弧がターゲットノードについていた場合は省く
          if next_edge.target_node.visited_edge.contains_key(next_edge.id) {
            log(debug, "edge cut");
            cont;
          }
        }

        heap::push(pq, (next_cost, (next_edge.target_node, next_edge.id)));
        #debug("push (%d, %s)", next_cost, (*next_edge.target_node).inspect());
      }
      
      log(debug, #fmt("heap size : %u", pq.nelts));
    }
    
  }

  ret result::ok("success");
}


#[doc="Return the shortest path with dijkstra"]
fn dijkstra_path(g: graph::graph, via: [@node]) -> result<[@node], str> {
  alt dijkstra_search(g, via, shortest) {
   ok(_) {
    
    let mut answer : [@node] = [];
    let mut current_node = vec::last(via);

    answer += [current_node];
    while vec::is_not_empty(current_node.hist) {
      
      let fastest_hist = current_node.hist[0u];
      assert(tuple::second(fastest_hist) == 0);

      current_node = g.edge_from_id(tuple::first(fastest_hist)).source_node;
      answer += [current_node];
    }
    ret result::ok(vec::reversed(answer));
   }
   err(m){
    ret result::err(m);
   }
  }
}

#[doc="return the shortest n path with dijkstra"]
fn dijkstra_n_path(g: graph::graph, via: [@node], answer_count: uint) -> result<[[@node]], str> {
  alt dijkstra_search(g, via, widespread) {
   err(m) {
    ret result::err(m);
   }
   ok(_) {
    // graph::dump_file(g, "tmp.graphml");

    let start_node = via[0u];
    let goal_node = vec::last(via);
    
    #info("goal_node:%s -> start_node:%s", goal_node.value, start_node.value);
    
    const max_cost_value : uint = 1u << 28;

    type tree_node_index = uint;

    #[doc="結果木を構成するノード"]
    type result_tree_node = {
      index: tree_node_index, 
      mut cost: uint, 
      mut upper_cost: uint, 
      mut lower_cost: uint, 
      mut delaytimes: [(@edge, uint)], 
      mut siblings: [mut tree_node_index],
      mut parent: option<tree_node_index>, 
      actual_edge: @edge
    };
    
    #[doc="結果木"]
    type result_tree = @{
      mut size: uint, 
      g: graph::graph, 
      mut nodes : [@result_tree_node], 
      mut visited_nodes : map::hashmap<node_value, bool>
    };

    impl tree_util for result_tree {
      
      fn node(idx: uint) -> @result_tree_node {
        ret self.nodes[idx];
      }

      #[doc="指定ノードより下につづく可能性のあるノードのうち最小のコストを設定"]
      fn update_lower_cost(node: @result_tree_node) {
        /*
                o <- 引数のnode
               / \    ------
              /   \    * * * <- nodeの到達した弧(node.delaytimes)
             /     \  ------
            /       o <- 以前に作成された結果木ノード(node.siblings)
           /
          o <- nodeの到達した弧をもとにつぎにつくられるノード
        
        node.siblings.lower_cost
        次に作られる弧で最小をセット
        */
        
        let (sibling_node, sibling_cost) = if vec::is_empty(node.siblings) {
          (none, max_cost_value)
        } else {
          let sib = self.node(node.siblings.head());
          (some(sib), sib.lower_cost)
        };
        
        let (next_node_label, next_cost) = if vec::is_empty(node.delaytimes) {
          ("<none>", max_cost_value)
        } else {
          let (next_edge, cost) = node.delaytimes.head();
          ((*next_edge).label(), cost)
        };
        
        #info("update lower_cost targetnode(%s) compare (sibling[%s]: %u, next[%s]: %u)", 
              (*node).label(), 
              sibling_node.map_default("<none>"){|s| (*s).label() }, 
              sibling_cost,
              next_node_label, 
              next_cost);

        node.lower_cost = node.cost + uint::min(sibling_cost, next_cost);
      }
      
      #[doc="指定ノードより上のノードのうち最小のコストを設定"]
      fn update_upper_cost(node: @result_tree_node) {
        /*
                o <- 親
               / \    ------
              /   \    * * * <- 親の到達した弧
             /     \  ------
            /       o <- 以前に作成された結果木ノード(親.sibling)
           /
          o <-- 引数のnode
         
        親に到達した弧の最小コスト(つぎにつくられそうな子ノード
        親.siblingのlower_cost
        親のupper_cost - 親のcost
        のみっつのうち最小のものを引数nodeのupper_costとする    
        */
        if option::is_some(node.parent) {
          let parent_node = self.node(node.parent.get());
          
          let (sibling_node, sibling_cost) = if vec::is_empty(parent_node.siblings) {
            (none, max_cost_value)
          } else {
            let sib = self.node(parent_node.siblings.head());
            (some(sib), sib.lower_cost)
          };
          
          // 親が次につくるノード
          let (next_sibling_label, next_sibling_cost) = if vec::is_not_empty(parent_node.delaytimes) {
            let (edge, cost) = parent_node.delaytimes.head();
            ((*edge).label(), cost)
          } else {
            ("<none>", max_cost_value)
          };

          // 親のupper_cost - 消費したcost
          let extended_parent_upper_cost = parent_node.upper_cost - parent_node.cost;
          
          // assert(node.siblings != parent_node.siblings);
          node.upper_cost = uint::min(uint::min(sibling_cost, next_sibling_cost), extended_parent_upper_cost);     
          #info("update upper_cost targetnode(%s) compare (parent[%s]:%u, parent.sibling[%s]:%u, parent.next_sibling[%s]:%u)",
                (*node).label(),
                (*parent_node).label(),
                extended_parent_upper_cost, 
                sibling_node.map_default("<none>"){|s| (*s).label()}, 
                sibling_cost,
                next_sibling_label, 
                next_sibling_cost);
        }
      }
      
      fn new_node(edge: @edge, 
                  cost: uint, 
                  parent: option<tree_node_index>) -> @result_tree_node {
        
        let new_node = @{
          index: self.size, 
          mut cost: cost, 
          mut upper_cost: max_cost_value, 
          mut lower_cost: max_cost_value, 
          mut delaytimes: vec::map(edge.source_node.hist){|hist|
            let (edg_id, cost) = hist;
            (self.g.edge_from_id(edg_id), cost as uint)
          }, 
          mut siblings: [mut], 
          mut parent: parent, 
          actual_edge: edge
        };
        
        self.nodes += [new_node];
        self.size += 1u;
        
        self.update_lower_cost(new_node);
        self.update_upper_cost(new_node);

        ret self.nodes[self.size-1u];
      }

      fn is_visited_node(node: @node) -> bool {
        ret self.visited_nodes.contains_key(node.value);
      }
      
      fn visit_node(node: @node) {
        #info("visite node: %s", node.value);
        self.visited_nodes.insert(node.value, true);
      }

      fn clear_visit(node: @node) {
        #info("clear visit status node: %s", node.value);
        self.visited_nodes.remove(node.value);
      }
      
      #[doc="Register sibling_node to target_node"]
      fn update_sibling(target_node: @result_tree_node, sibling_node: @result_tree_node) {
        if !target_node.siblings.contains(sibling_node.index) {
          #info("append sibling target_node[%s] and its siblings: %s", 
                (*target_node).label(), target_node.siblings.foldl(""){|acc, a| acc+#fmt("%s,", (*self.node(a)).label())});
          
          target_node.siblings += [sibling_node.index];
        }
      }
    }
    
    impl tree_node_util for result_tree_node{
      fn label() -> str{
        ret #fmt("(i:%u,%s", self.index, (*self.actual_edge).label());
      }

      fn inspect() -> str{
        
        fn concat(&&ac: str, &&a: (@edge, uint)) -> str {
          ret ac+#fmt("(%s,%u),", (*tuple::first(a)).label(), tuple::second(a));
        }
        
        fn inspect_node(&&node_index: tree_node_index) -> str {
          ret #fmt("%u", node_index);
        }

        ret #fmt("i:%u v:%s cost:%u upper:%u lower:%u parent:%s siblings:%s delaytimes:[%s]", 
                 self.index, 
                 (*self.actual_edge).label(),
                 self.cost,
                 self.upper_cost,
                 self.lower_cost, 
                 self.parent.map_default("none", inspect_node), 
                 #fmt("[%s]", self.siblings.foldl("") {|ac, a| ac+#fmt("%u,", a)}), 
                 vec::foldl("", self.delaytimes, concat));
      }
    }

    let tree = @{
      mut size: 0u, 
      g: g, 
      mut nodes: [], 
      mut visited_nodes: map::str_hash::<bool>()
    };
    
    // goal_nodeを起点とする仮想出口ノードを登録
    let goal_tree_node = tree.new_node(
      graph::edge(goal_node, graph::node(uint::max_value, ""), int::max_value), 
      0u, 
      none);
    
    let mut current_index = goal_tree_node.index;
    let mut answers : [[@node]] = [];
    
    while !(tree.nodes[current_index].upper_cost >= max_cost_value && tree.nodes[current_index].lower_cost >= max_cost_value){
      let current_node  = tree.node(current_index);
      #info("- - - - current_node: %s", (*current_node).inspect());
      
      if current_node.upper_cost >= current_node.lower_cost {
        #info("down down down");
        
        let mut sibling_cost = if vec::is_empty(current_node.siblings) {
          max_cost_value
        } else {
          tree.node(current_node.siblings.head()).lower_cost
        };
        
        if vec::is_not_empty(current_node.delaytimes) && current_node.delaytimes.head().second() <= sibling_cost {

          let (next_edge, cost) = vec::shift(current_node.delaytimes);
          // 遅延時間リストのなかで閉路になりうる候補をけずっとく
          while current_node.delaytimes.len() > 0u && tree.is_visited_node(current_node.delaytimes.head().first().source_node) {
            vec::shift(current_node.delaytimes);
          }
          
          if tree.is_visited_node(next_edge.source_node) {
            // 次が閉路(すでに手繰られてる)
            
            #info("next edge is already visited, so update lower_cost and continue.");
            tree.update_lower_cost(current_node);
            cont;

          }

          if next_edge.source_node == start_node {
            #debug(".:*:.:*:.:*:.:*:.:*:.:*:.:*:find answerrr.:*:.:*:.:*:.:*:.:*:.:*:.:*:.:*:.:*:");

            // 答えに追加して、前のインデックスのsibling更新
            // 次の到着弧か以前たどった子の小さい方を設定
            // currentはそのまま次へ
            let mut answer : [@node] = [start_node];
            let mut answer_node = current_node;
            while true {
              // #info("ans:%s", answer_node.actual_edge.source_node.value);
              answer += [answer_node.actual_edge.source_node];
              alt answer_node.parent {
               some(ix) {
                answer_node = tree.node(ix);
               }
               none { break; }
              }
            }
            
            #info("answer: %s", answer.foldl(""){|acc, a| acc+#fmt("%s -> ", a.value)});
            
            #debug(".:*:.:*:.:*:.:*:.:*:.:*:.:*:find answerrr.:*:.:*:.:*:.:*:.:*:.:*:.:*:.:*:.:*:");

            answers += [answer];
            if vec::len(answers) >= answer_count {
              break;
            }
            tree.update_lower_cost(current_node);
            
          } else {
            
            #info("create new node");
            
            let next_node = tree.new_node(next_edge, cost, some(current_node.index));
            #info("next_tree_node: %s", (*next_node).inspect());
            tree.visit_node(next_edge.source_node);
            
            current_index = next_node.index;
          }

        } else {
          
          #info("goto previous sibling node");

          fn le(&&a: @result_tree_node, &&b: @result_tree_node) -> bool {
            a.lower_cost < b.lower_cost
          }
          let next_siblings = sort::merge_sort(le, vec::map(vec::from_mut(current_node.siblings)) {|s| tree.node(s) });

          #info("sorted siblings: %s", next_siblings.foldl(""){|acc, s| 
            acc+#fmt("%s | ", (*s).inspect())
          });
          
          current_node.siblings = vec::to_mut(next_siblings.map{|s| s.index});
          
          #debug("current siblings: %s", current_node.siblings.foldl(""){|acc, s| 
            acc+#fmt("%s | ", (*tree.node(s)).inspect())
          });


          let jump_sibling_node = tree.node(current_node.siblings.head());
          assert(current_node.lower_cost == jump_sibling_node.lower_cost);
          
          tree.update_upper_cost(jump_sibling_node);
          tree.update_lower_cost(jump_sibling_node);
          
          current_index = jump_sibling_node.index;
        }
        
        
      } else {
        #info("up up up");
        
        let parent_node = tree.node(current_node.parent.get());
        tree.clear_visit(current_node.actual_edge.source_node);
        /* 上の結果木ノードに最短経路がふくまれているので、backtrack
        その道上で次の最短時間のヒントを収集していく

                  根
                 /
                o
               / \
              o   o <- ｺｺﾆｻｲﾀﾝﾛ
             /
            o <- ｲﾏｺｺ
        */

        tree.update_sibling(parent_node, current_node);
        tree.update_lower_cost(parent_node);
        current_index = parent_node.index;
      }
    }
    
    ret result::ok(answers);
   }
  }
}