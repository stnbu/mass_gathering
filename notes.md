
## Collisions

### Possible Combinations

* Planet <-> Projectile
* Planet <-> Planet
* Projectile <-> Projectile [not handled for now]

"Order" does not matter here. A collision event only includes a pair of colliders and some flags. There is no sense of "direction".

Nothing else should be a `Collider`. If anything else gets `Collider` (as in, someone alters the creation of an entity and adds `Collider`),
the above list needs to be expanded.

### Systems

We only consider `CollisionStarted` events (is ok, yes?)

Assume that `n`-way collisions might happen. Suppose that `p1`, `p2`, `p3`, `m1`, `m2` all exist. `p` for planet `m` for "missile" (projectile). Assume they show up in our events like this:

```
[
    (p1, p2),
	(p3, m1),
	(p3, p2),
	(p2, m2),
]
```

The graph looks like this:

```
    m2  m1
    |   |
p1--p2--p3

```

Are these all associative? Commutative? Even for some arbitrary attribute that you plan to combine between planets in some arbitrary way?

The thing to do, maybe, is define types for all the "things" that are transferred or combined, then show that they have the mathematical properties you want.

#### Planet-Planet

The physics stuff is obvious: The "major" planet (the one that carries the identity of the newly merged planet) should have its physics updated using those of the minor (to be despawned) planet. Note the "storage" of data for planet entities:

* Position is stored in `Transform.translation` **Rotation is ignored for planets.** They are smooth spheres with infinite symmetry.
* Velocity and mass are stored in `Momentum`.
* Radius is set for `Isoshere` and `Collider` but is not retrieved. Instead radius is calculated from mass (and visa versa as needed). **Density is always `1.0`.**
* Color/material is currently random and has no meaning. When planets merge, the major planet's color is unchanged. Hence, effectively, the "new" planet has the same color (material) of the larger of the two colliding planets. This will change and become more complex!

The last point brings us to: not physics stuff.

Other metadata that could be included with a planet entity:

* The "owner" of the planet. This will probably be `Option<Player>` or something, with `None` being the starting value. (In other words, owned by a particular player or not owned at all.)
* Color, Texture and other optical parameters. This could even included animated clouds or maybe sparkling bodies of water.
* Incoming projectiles: wherever we store the planet's entity, incoming projectiles must be dealt with when merging, especially for the minor planet.
* If relevant: Markup entities (breadcrumbs, floating vector helpers.)

#### Planet-Projectile

#### Relationship of the two colliders

Three-way collision: planet-planet-projectile, do they exist? My attempt at a definition:

> A three-way 