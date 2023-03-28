import json
from sentence_transformers import SentenceTransformer, util


def get_embed(sentence):
    return model.encode(sentence, convert_to_tensor=True)


def get_cos_sim(embedding_1, embedding_2):
    return util.pytorch_cos_sim(embedding_1, embedding_2)  # type: ignore


def check(data):
    base_sentence = data["base_sentence"]
    sentences = data["sentences"]

    # Compute embedding for both lists
    base_embedding = get_embed(base_sentence)
    embeddings = [get_embed(sentence) for sentence in sentences]

    result = []

    # Compute and print similarities.
    for i, embed in enumerate(embeddings):
        similarity = get_cos_sim(base_embedding, embed)
        result.append([sentences[i], similarity.item()])

    # 類似度の高い順にソート
    result.sort(key=lambda x: x[1], reverse=True)

    # 結果を表示
    print(f"基準文: {base_sentence}")
    for i, res in enumerate(result):
        print(f"{i+1}位: {res[0][0:30]} ({res[1]})")


# JSONファイルからデータを読み込む
with open("sentences.json", "r") as file:
    data = json.load(file)

model = SentenceTransformer("sentence-transformers/all-MiniLM-L6-v2")

for d in data:
    check(d)
    print()
