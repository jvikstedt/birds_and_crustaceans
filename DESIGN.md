# Birds and Crustaceans | Design

Multiplayer game where players will try to kill as many Birds and Crustaceans as possible.

**These are the initial design ideas and can and probably will change during the development week.**

## Goal
My goal is to make an multiplayer game. I have been working on the Bevy engine for few months now and I have already implemented some working networking code that I'm planning to reuse here. Networking code is probably overkill and not necessary most suitable for this game idea, however it will provide me good information about how the code will function in more real usage.

The idea of Birds And Crustaceans is altered version of game idea that I have been thinking about for many years now, its somewhat inspired by Rabbit vs Sheep custom map made in Warcraft 3.

Since the Jam will last only for one week and I have limited time, I have to keep the scope small.

## Basics

Once the game starts, Birds and Crustaceans will start to spawn. Players will be able to move their cursor and start clicking on the Birds and Crustaceans to damage them. 

Birds and Crustaceans should have some different behavior. Maybe Birds would be faster ( harder to hit ). Crustaceans would be slower but has more health.

I would like to add different abilities and features that user can do to improve their power. However, will have to see if I have enough time to implement all of those.

## Matchmaking
For simplicity sake, I was thinking about just allowing one active game to run at once. Either allow players to join during the match or just wait for the next match.

One game should allow unlimited amount of players to join or at least allow many players. Scale amount of spawning entities based on player count. 

## Jam theme
**Unfair Advantage** was selected as a theme for the Jam. My plan is to implement some temporal buffs or actions that players can try to get. Those buffs should feel powerful and to have feeling of **Unfair Advantage**.
