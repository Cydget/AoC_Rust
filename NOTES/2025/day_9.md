[LANGUAGE: Rust]
I had to spend a while thinking before I wrote any code, but I think I came up with a decent solution.
They give us a closed polygon point boundary. I standardized its winding direction to always have clockwise winding.
We know that the solution must start and end on a red tile with the other corners easily computed. My input was about 500 points long, so 500Choose2 = 124,750 solutions to individually check.
I ended up determining there were only four unique ways the Start ( S ) and End (E) could be. Example of 1 of the 4
SC
CE
To see if it was valid I needed to check just 4 conditions. If any failed, we can early return and try validating the next. 
Can Start Go Clockwise Right to C1 and CounterClockWise Down to C2?
Can End Go CounterClockWise Up to C1 and Clockwise Left to C2?
Because we were given a boundary, and we know the index of (Start Corner or End Corner) on that boundary, we can simply walk along the boundary starting at the exact index. 
If we are searching for a CW point to the right, and along our walk on the boundary we go past the required x position, we know we can hit that point. However, we must stop walking if we ever make a turn that would "build a wall" in front of our corner. In this example. If we went Down past C1's y position our boundary would have cut off the direct line to C by building a wall in front of it.
Walking around the boundary is easy by increasing your offset from where ever you started from. If you are trying to walk the boundary counterclockwise, just start subtracting your offset. 
The boundary is only 500 points long, but because it is really easy to build a wall/exit early, we don't end up checking that many before we move on to the next rect to validate. 

Still ended up being a lot of code/ cases to check, but compute time is < 1 second including svg generation. I have a few ideas on how it can be made faster but I'm not one to chase milliseconds. 
Also, I'm sure there are issues with my code, but at least it gave me my solution. 
I had spent some time thinking that I needed to travel from each corner clockwise, but was having issues finding the nearest boundary point to start my walk if I started from a C corner and had to walk to (S/E), Allowing myself to search counter clockwise removed the requirement from starting at an "unknown" boundary index. 
Code: Git
Image: SVG  ( only showing solutions that were largest at time of find ) 