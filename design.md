The goal of this project is to combine the replayability of roguelikes with the separate screen turn-based combat of CRPGs/JRPGs. The current competition in this space is from rom hacks of 2D JRPGs, like the various final fantasy randomizers. I think I can do much better, primarily because it's much easier to add content.


World Design
============
1. Moving around and interacting with the world should feel familiar to anyone that's played a rogue-like.
2. Each area will have a limited number of random battles, so grinding is possible but it's not required.
3. There will be a main "overworld" that is generated. Tiles on the overworld can be areas that are entered by pressing '>' like going down in a roguelike.
4. "Towns" will most likely be a menu that lists services. Maybe talk to NPCs that tell you about the world.
5. Don't let players "sell" items, I think DCSS was right about that.
6. I plan to require some amount of inventory management. Most likely I'll go with just item weight, and have characters contribute their str to a shared party inventory.


Characters
==========
1. Each party will have [TODO: 4-6] characters.
2. Lots of character classes with unique abilities. At a minimum I want as many classes as Everquest.
3. Inspiration for classes include MUDs, DnD, Everquest, and roguelikes.
4. Classes have a combat score from 1-5 that directly impacts their melee ability, stolen from MajorMUD
5. Characters use hitpoints, mana, and stamina.
5. Classes:
             |Combat|Scribe        |Scrolls      |Spells at Level|Armor |Weapons |Other                                           |
             |------|--------------|-------------|---------------|------|--------|------------------------------------------------|
      Wizard |  1   |All Arcane    |All Arcane   |2 Arcane       |None  |        |
      Priest |  1   |All Divine    |All Divine   |2 Divine       |None  |        |
       Druid |  1   |All Nature    |All Nature   |2 Nature       |Light |        |
     Scholar |  1   |All           |All          |None           |Light |        |
      Cleric |  3   |Most Divine   |All Divine   |1 Divine       |Heavy |        |
        Monk |  4   |None          |None         |None           |Light |        |Bonus HP and Dodge




  - Wizard: Combat 1, can scribe any arcane spells, can use any arcane scrolls, gains 2 new arcane spells at level up.
  - Priest: Combat 1, can scribe any divine spells, can use any divine scrolls, gains 2 new divine spells at level up.
  - Druid: Combat 1, can scribe any nature spells, can use any nature scrolls, gains 2 new nature spellls at level up.
  - Scholar: Combat 1, can scribe any arcane, divine, or nature spells. Light armor, No new spells at level up.
  - Cleric: Combat 2, can scribe any divine spells, can use any divine scrolls, heavy armor


Magic
=====
1. Spells will either be divine, arcane, or nature magic.
2. DnD style scrolls, where they can either be learned by "pure" casters of the Divine/Arcane/Nature type or cast by partial users of those spell schools.


