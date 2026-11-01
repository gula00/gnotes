# mapreduce
@ray.remote
def map_count(text_chunk: str):
    cnt = collections.Counter()
    for w in text_chunk.lower().split():
        cnt[w] += 1
    return cnt

@ray.remote
def reduce_merge(a: collections.Counter, b: collections.Counter):
    a.update(b)
    return a

texts = [
    "Ray makes distributed computing simple simple",
    "Ray tasks are easy to scale",
    "Remote functions return ObjectRefs",
    "ObjectRefs build a DAG of dependencies"
]

# Map 阶段（并行）
parts = [map_count.remote(t) for t in texts]

# Reduce 阶段（树形合并，依赖于前序结果）
# [(0, 1), (2, 3)] => [((0, 1), (2, 3))]
while len(parts) > 1:
    nxt = []
    for i in range(0, len(parts), 2):
        if i+1 < len(parts):
            nxt.append(reduce_merge.remote(parts[i], parts[i+1]))
        else:
            nxt.append(parts[i])
    parts = nxt

final_counter = ray.get(parts[0])
print(final_counter.most_common(5))

ray.shutdown()
