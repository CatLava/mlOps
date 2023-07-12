from textblob import TextBlob
import wikipedia

def search_wikipedia(name):
    '''Search wikipedia'''
    return wikipedia.search(name)

def summarize_wikipedia(name):
    '''summarize wikipedia'''
    return wikipedia.summary(name)

def get_text_blob(text):
    '''Get text blob'''
    blob = TextBlob(text)
    return blob

def get_phrases(name):
    '''Find Wikipedia name and return back phrases'''
    text = summarize_wikipedia(name)
    blob = get_text_blob(text)
    phrases = blob.noun_phrases
    return phrases
    
gsw = wikipedia.summary("Golden State Warriors")
