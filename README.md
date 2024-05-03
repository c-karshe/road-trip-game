This is my Road Trip Game!

The goal of this game was to set up a Road Trip, roughly inspired by Oregon Trail. I wanted to start off with a shop that would
allow the player to buy all the resources they needed for the trip. Then, there would be a series of events in randomizded order,
and if any of the key resources ran out then there would be a game over.

First I imported the different things I needed and then set up the different structs necessary for the game. After this I 
created a vector of events to shuffle in order to get a random order. Then I set up the shop, printing out the different
item descriptions as well as allowing the user to type in the amount of gas, snacks, and vibes they wanted. The minimum to start
the game was 1 of each, however starting off with just that would make for a pretty difficult playthrough! Here the budget can also
be changed to raise or lower the difficulty level.

After ensuring the user purchased an amount of resources that wasn't over budget, the game began. For some events, there were options
where the player could make a choice that required a certain minimum of resources. This cost would be printed out next to the choice,
and the player is unable to select that choice unless they have the minimum amount of resources. Once a choice was made, the effect
of this choice was applied to all of the resources and the next event was selected. If the player ran out of gas, snacks, or vibes 
a specific message would be printed out alongside the game over message. otherwise, if you made it to the end a congratulations message
was printed.

As for the structure of the main file, it primarily just contained the structure of the map. Here you can see in more detail
the way each option had effects on the resources, and the options that had minimum resource number required. Place like the gas
station and roadside attractions required money, but gave vibes, snacks, or gas in return. The basic cost of moving from one event
to the next was 1 gas. Finally, in the Map structure is where I set the price of each resource.
