## try-embedding

embeddingをやってみます。

### 環境構築手順

```console 
$ python3 -m venv venv
$ source venv/bin/activate
$ pip install -r requirements.txt
```

```console
pip install -U sentence-transformers
pip install flake8 black autopep8 pytest
pip freeze > requirements.txt
touch main.py
touch .gitignore
```
