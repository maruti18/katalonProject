<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>reqrt</name>
   <tag></tag>
   <elementGuidId>1c9433d7-03ca-483a-9838-0d4195068481</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://reqres.in//api/users?delay=3</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

String[] arrayResponse = [&quot;why&quot;, &quot;hello&quot;, &quot;there&quot;]
String[] arrayExpect = [&quot;there&quot;, &quot;why&quot;, &quot;hello&quot;]
assertThat(arrayResponse).containsOnly(&quot;there&quot;, &quot;hello&quot;, &quot;why&quot;)
assertThat(arrayResponse).containsOnlyElementsOf(Arrays.asList(&quot;why&quot;, &quot;there&quot;, &quot;hello&quot;))

assertThat(arrayResponse).containsExactly(&quot;why&quot;, &quot;hello&quot;, &quot;there&quot;)
assertThat(arrayResponse).containsExactlyElementsOf(Arrays.asList(&quot;why&quot;, &quot;hello&quot;, &quot;there&quot;))

assertThat(arrayResponse).containsSequence(&quot;why&quot;, &quot;hello&quot;)
assertThat(arrayResponse).containsSubsequence(&quot;why&quot;, &quot;there&quot;)
assertThat(arrayResponse).containsAnyOf(&quot;why&quot;, &quot;nothing&quot;, &quot;new&quot;)

assertThat(arrayResponse).contains(&quot;hello&quot;, atIndex(1))

WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

WS.verifyElementPropertyValue(response, 'issues[0].fields.project.key', 'KTP')

assertThat(jsonResponse.issues[0].fields.project.key).isEqualTo('KTP')

WS.verifyElementPropertyValue(response, 'issues[0].fields.project.key', 'KTP')

assertThat(jsonResponse.issues[0].fields.project.key).isEqualTo('KTP')

assertThat(response.getResponseText()).isEqualTo(&quot;Katalon Test Project&quot;)

assertThat(response.getHeaderFields().get('Content-Type').toString()).isEqualTo('[application/json;charset=UTF-8]')

assertThat(response.getHeaderFields().containsKey('Content-Type')).isTrue()

assertThat(response.getResponseText()).contains('Olten')
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
