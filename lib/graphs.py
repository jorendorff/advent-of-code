import heapq
from collections import namedtuple

State = namedtuple('State', 'total_weight prev_state prev_edge node')


def _state_to_path(state):
    rpath = [state.node]
    while state.prev_state is not None:
        rpath.append(state.prev_edge)
        state = state.prev_state
        rpath.append(state.node)
    rpath.reverse()
    return rpath


def breadth_first_weighted_paths(start_set, destination_test, ways, key=lambda x: x):
    # `queue` is a binary heap of nodes to explore.
    # `seen` tracks nodes we have already visited.
    start_set = list(start_set)
    queue = [State(0, None, None, node) for node in start_set]
    heapq.heapify(queue)
    seen = {key(entry[1]): entry for entry in queue}

    while queue:
        state = heapq.heappop(queue)
        if destination_test(state.node):
            yield state.total_weight, _state_to_path(state)

        for edge, weight, after_node in ways(state.node):
            k = key(after_node)
            after_state = State(
                state.total_weight + weight,
                state,
                (edge, weight),
                after_node
            )

            if k in seen:
                # Already been here. Do nothing. Unless...
                known_state = seen[k]
                if after_state.total_weight < known_state.total_weight:
                    # Oh wow, we found a quicker route! Update the queue.
                    where = queue.index(known_state)
                    queue[where] = seen[k] = after_state  # replace old state with new one
                    heapq.heapify(queue)  # totally resort
            else:
                heapq.heappush(queue, after_state)
                seen[k] = after_state


def shortest_weighted_path(start_set, destination_test, ways, key=lambda x: x):
    for tw, path in breadth_first_weighted_paths(start_set, destination_test, ways, key):
        return tw, path
    else:
        return None
