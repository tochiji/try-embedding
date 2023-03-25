from sentence_transformers import SentenceTransformer, util

def get_embed(sentence):
    return model.encode(sentence, convert_to_tensor=True)

def get_cos_sim(embedding_1, embedding_2):
    return util.pytorch_cos_sim(embedding_1, embedding_2)  # type: ignore

base_sentence = ["明石家さんまが会場で爆笑をさらった"]
sentences = [
    ["ビートたけしが舞台を盛り上げた"],
    ["お笑い芸人は人を笑わせることが仕事だ"],
    ["安倍晋三は日本の元内閣総理大臣です"],
    ["松本人志はお笑いに関する映画を撮影した"],
    ["イヌは散歩する"],
    ["間寛平が一発ギャクを披露した"],
    ["ジミー大西はよくお笑い番組に出演している"],
    ["タモリのミュージックステーションは面白い"],
    ["みずほ銀行は日本の金融機関だ"],
    ["しょうゆうことという明石家さんまのギャグが存在する"],
    ["杉本高文は明石家さんまの本名です"],
]

model = SentenceTransformer("sentence-transformers/all-MiniLM-L6-v2")

# Compute embedding for both lists
base_embedding = get_embed(base_sentence)
embeddings = [get_embed(sentence) for sentence in sentences]

# Compute and print similarities.
for i, embed in enumerate(embeddings):
    similarity = get_cos_sim(base_embedding, embed)
    print(f"{base_sentence} と {sentences[i]} の類似度:\t\t {similarity}")

