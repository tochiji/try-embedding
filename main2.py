import json
from sentence_transformers import SentenceTransformer, util
import sys

model = SentenceTransformer("sentence-transformers/all-MiniLM-L6-v2")  # type: ignore


def get_embed(sentence):
    return model.encode(sentence, convert_to_tensor=True)


def get_cos_sim(embedding_1, embedding_2):
    return util.pytorch_cos_sim(embedding_1, embedding_2)  # type: ignore


if len(sys.argv) > 1:
    arg1 = sys.argv[1]
else:
    print("第一引数がありません")
    sys.exit(1)

# JSONファイルからデータを読み込む
with open("result.json", "r") as file:
    data = json.load(file)

for d in data:
    title = d["title"]
    contents = d["contents"]
    text = "title: " + title + "\n contents:" + contents
    embed = get_embed(text)
    d["embed"] = embed

search_embed = get_embed(arg1)

result = []

for d in data:
    similarity = get_cos_sim(search_embed, d["embed"])
    result.append([d["title"], similarity.item()])

# 類似度の高い順にソート
result.sort(key=lambda x: x[1], reverse=True)

# 結果を表示
print(f"基準文: {arg1}")
for i in range(20):
    print(f"{i+1}位: {result[i][0]} ({result[i][1]})")
