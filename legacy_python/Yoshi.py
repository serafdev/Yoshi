import configparser
import discord
from discord.ext import commands
from discord.ext.commands import Bot
import asyncio

client = discord.Client()

@client.event
async def on_ready():
    print('Logged in as')
    print(client.user.name)
    print(client.user.id)
    print('-----')

@client.event
@asyncio.coroutine
def on_message(message):
    if message.author == client.user: return
    if str(message.channel) == 'articles' and not 'http' in message.content.lower():
        yield from client.send_message(message.channel, f'Hey {message.author}, refrain from talking here, thank you.')
        yield from client.send_message(client.get_channel('554422748496134165'), f'A message that was not an article was sent in `articles`:\n  ```{message.author}: {message.content}```')
        


def main():
    config = configparser.ConfigParser()
    config.read('key.ini')
    client.run(config['Discord']['key'])

if __name__ == '__main__':
    main()