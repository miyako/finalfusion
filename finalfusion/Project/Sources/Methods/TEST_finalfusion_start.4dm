//%attributes = {"invisible":true}
var $finalfusion : cs:C1710.server
$finalfusion:=cs:C1710.server.new()

$isRunning:=$finalfusion.isRunning()

var $model : 4D:C1709.File
$model:=Folder:C1567(Folder:C1567("/PACKAGE/").platformPath; fk platform path:K87:2).parent.file("models/GloVe/glove.300d.fifu")

$finalfusion.start({model: $model; port: "8080"})