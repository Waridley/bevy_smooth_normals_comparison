A demo of the difference between Bevy's current implementation of smooth 
normals, and an implementation that ignores face area.

In all of the following screenshots, bevy's current main-branch implementation
is on the left, and mine is on the right. The lights are aiming straight down
along the -Y axis.

### Top
![](screenshots/top.png "Top view")

Notice the maximum brightness is on the left face of the spire of the left
mesh, but perfectly centered on the peak of the right spire.

### Front
![](screenshots/front_left.png "View from front left")

![](screenshots/front_right.png "View from front right")

![](screenshots/front_gizmos.png "Front view with normal gizmos")

This last screenshot shows the vertex normals using gizmo arrows. In my
implementation, the peak normal is pointing straight up.

However, this view does highlight another potential drawback of this approach:
The normal in the concave corner where the gambrel meets the spire is almost
entirely influenced by the spire face and not the ridge of the gambrel. It's
unclear that that is strictly incorrect, but since the far left vertex of the
ridge of the gambrel is pointing significantly more in the upward direction,
scaling the mesh along the normals would result in a more tilted ridge. It also
results in a more pronounced faux-ambient occlusion effect.

Ultimately, this geometry is doomed to have ugly smooth lighting and would 
really do better with flat normals (it gets even worse if I add the bottom 
faces). However, I think the spire peak being perfectly vertical is by far the 
best option.
