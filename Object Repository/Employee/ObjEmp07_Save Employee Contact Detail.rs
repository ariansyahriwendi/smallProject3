<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>ObjEmp07_Save Employee Contact Detail</name>
   <tag></tag>
   <elementGuidId>eb3d490d-43ab-460e-9d5a-b7706eb81f92</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;application/x-www-form-urlencoded&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;id&quot;,
      &quot;value&quot;: &quot;273&quot;
    },
    {
      &quot;name&quot;: &quot;addressStreet1&quot;,
      &quot;value&quot;: &quot;Umalas&quot;
    },
    {
      &quot;name&quot;: &quot;addressStreet2&quot;,
      &quot;value&quot;: &quot;Canggu&quot;
    },
    {
      &quot;name&quot;: &quot;city&quot;,
      &quot;value&quot;: &quot;Badung&quot;
    },
    {
      &quot;name&quot;: &quot;state&quot;,
      &quot;value&quot;: &quot;Bali&quot;
    },
    {
      &quot;name&quot;: &quot;zip&quot;,
      &quot;value&quot;: &quot;80361&quot;
    },
    {
      &quot;name&quot;: &quot;country&quot;,
      &quot;value&quot;: &quot;Indonesia&quot;
    },
    {
      &quot;name&quot;: &quot;homeTelephone&quot;,
      &quot;value&quot;: &quot;02112345&quot;
    },
    {
      &quot;name&quot;: &quot;mobile&quot;,
      &quot;value&quot;: &quot;081000000110&quot;
    },
    {
      &quot;name&quot;: &quot;workTelephone&quot;,
      &quot;value&quot;: &quot;02134567&quot;
    },
    {
      &quot;name&quot;: &quot;workEmail&quot;,
      &quot;value&quot;: &quot;halotest273@mailinator.com&quot;
    },
    {
      &quot;name&quot;: &quot;otherEmail&quot;,
      &quot;value&quot;: &quot;halotest274@mailinator.com&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>x-www-form-urlencoded</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/x-www-form-urlencoded</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${GlobalVariable.access_token}</value>
   </httpHeaderProperties>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.baseURL}/api/v1/employee/273/contact-detail</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
