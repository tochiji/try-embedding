import json
from sentence_transformers import SentenceTransformer, util

def get_embed(sentence):
    return model.encode(sentence, convert_to_tensor=True)

def get_cos_sim(embedding_1, embedding_2):
    return util.pytorch_cos_sim(embedding_1, embedding_2)  # type: ignore

# JSONファイルからデータを読み込む
with open("sentences.json", "r") as file:
    data = json.load(file)

base_sentence = data["base_sentence"]
sentences = data["sentences"]

model = SentenceTransformer("sentence-transformers/all-MiniLM-L6-v2")

# Compute embedding for both lists
base_embedding = get_embed(base_sentence)
embeddings = [get_embed(sentence) for sentence in sentences]

# Compute and print similarities.
for i, embed in enumerate(embeddings):
    similarity = get_cos_sim(base_embedding, embed)
    print(f"{base_sentence} と {sentences[i]} の類似度:\t\t {similarity.item()}")

