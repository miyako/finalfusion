Class extends _CLI

Class constructor($controller : 4D:C1709.Class)
	
	If (Not:C34(OB Instance of:C1731($controller; cs:C1710._finalfusion_Controller)))
		$controller:=cs:C1710._finalfusion_Controller
	End if 
	
	Super:C1705("finalfusion-server"; $controller)
	
Function get worker() : 4D:C1709.SystemWorker
	
	return This:C1470.controller.worker
	
Function terminate()
	
	This:C1470.controller.terminate()
	
Function run($option : Object) : 4D:C1709.SystemWorker
	
	var $command : Text
	$command:=This:C1470.escape(This:C1470.executablePath)
	
	If (Value type:C1509($option.model)=Is object:K8:27) && (OB Instance of:C1731($option.model; 4D:C1709.File)) && ($option.model.exists)
		$command+=" --model "
		$command+=This:C1470.escape(This:C1470.expand($option.model).path)
	Else 
		return   //mandatory
	End if 
	
	If (Value type:C1509($option.port)=Is real:K8:4) && ($option.port#0)
		$command+=" --port "
		$command+=String:C10($option.port)
	End if 
	
	return This:C1470.controller.execute($command; Null:C1517; $option.data).worker