Class constructor
	
	var __WORKER__ : 4D:C1709.SystemWorker
	
Function run($option : Object)
	
	If ($option=Null:C1517) || (Value type:C1509($option)#Is object:K8:27)
		return 
	End if 
	
	var $signal : 4D:C1709.Signal
	$signal:=New signal:C1641("finalfusion")
	
	CALL WORKER:C1389($signal.description; This:C1470._run; $option; $signal)
	
	$signal.wait()
	
Function _run($option : Object; $signal : 4D:C1709.Signal)
	
	var $finalfusion : cs:C1710._finalfusion
	$finalfusion:=cs:C1710._finalfusion.new()
	
	If (OB Instance of:C1731(__WORKER__; 4D:C1709.SystemWorker)) && (Not:C34(__WORKER__.terminated))
		//already started
	Else 
		__WORKER__:=$finalfusion.run($option)
	End if 
	
	$signal.trigger()
	
Function _terminate($signal : 4D:C1709.Signal)
	
	If (OB Instance of:C1731(__WORKER__; 4D:C1709.SystemWorker)) && (Not:C34(__WORKER__.terminated))
		__WORKER__.terminate()
	End if 
	
	$signal.trigger()
	
Function terminate()
	
	var $signal : 4D:C1709.Signal
	$signal:=New signal:C1641("finalfusion")
	
	CALL WORKER:C1389($signal.description; This:C1470._terminate; $signal)
	
	$signal.wait()
	
Function _isRunning($signal : 4D:C1709.Signal)
	
	var $isRunning : Boolean
	
	If (OB Instance of:C1731(__WORKER__; 4D:C1709.SystemWorker)) && (Not:C34(__WORKER__.terminated))
		$isRunning:=True:C214
	End if 
	
	Use ($signal)
		$signal.isRunning:=$isRunning
	End use 
	
	$signal.trigger()
	
Function isRunning() : Boolean
	
	var $signal : 4D:C1709.Signal
	$signal:=New signal:C1641("finalfusion")
	
	CALL WORKER:C1389($signal.description; This:C1470._isRunning; $signal)
	
	$signal.wait()
	
	return $signal.isRunning